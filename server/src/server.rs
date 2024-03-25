/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2015-2023 Dariusz Depta
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2015-2023 Dariusz Depta
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::data::ApplicationData;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use dmntk_common::{ColorPalette, Jsonify};
use dmntk_feel::FeelScope;
use dmntk_workspace::Workspaces;
use std::borrow::Borrow;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::{env, io};

const DMNTK_DEFAULT_PORT: u16 = 22022;
const DMNTK_DEFAULT_HOST: &str = "0.0.0.0";
const DMNTK_HOST_VARIABLE: &str = "DMNTK_HOST";
const DMNTK_PORT_VARIABLE: &str = "DMNTK_PORT";
const DMNTK_DIR_VARIABLE: &str = "DMNTK_DIR";
const CONTENT_TYPE: &str = "application/json";

/// Handler for evaluating invocable identified
/// by unique name in namespace represented by RDNN.
#[post("/evaluate/{path:.*}")]
async fn evaluate(path: web::Path<String>, request_body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  let workspace: &Workspaces = data.workspaces.borrow();
  match dmntk_evaluator::evaluate_context(&FeelScope::default(), &request_body).and_then(|input_data| workspace.evaluate(&path, &input_data)) {
    Ok(value) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!(r#"{{"data":{}}}"#, value.jsonify())),
    Err(reason) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!(r#"{{"errors":[{{"detail":"{reason}"}}]}}"#)),
  }
}

/// Handler for 404 errors.
async fn not_found() -> HttpResponse {
  HttpResponse::NotFound().content_type(CONTENT_TYPE).body(r#"{"errors":[{"detail":"endpoint not found"}]}"#)
}

#[cfg(feature = "tck")]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(crate::tck::post_tck_evaluate);
}

#[cfg(not(feature = "tck"))]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(evaluate);
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>, opt_dir: Option<String>, colors: ColorPalette, verbose: bool) -> io::Result<()> {
  let application_data = web::Data::new(ApplicationData {
    workspaces: Arc::new(Workspaces::new(&get_root_dir(opt_dir), colors.clone(), verbose)),
  });
  let address = get_server_address(opt_host, opt_port);
  println!("{1}dmntk{0} {2}{address}{0}", colors.reset(), colors.blue(), colors.yellow());
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::PayloadConfig::new(4 * 1024 * 1024))
      .configure(config)
      .default_service(web::route().to(not_found))
  })
  .bind(address)?
  .run()
  .await
}

/// Returns the host address and the port number, the server will start to listen on.
///
/// The default host and port are defined by `DMNTK_DEFAULT_HOST` and `DMNTK_DEFAULT_PORT` constants.
/// When other values are given as parameters to this function, these will be the actual host and port.
/// Host and port may be also controlled using environment variables:
/// - `DMNTK_HOST` for the host name,
/// - `DMNTK_PORT` for the port name.
///
/// Priority (from highest to lowest):
/// - `opt_host` an `opt_port` parameters,
/// - `DMNTK_HOST` and `DMNTK_PORT` environment variables
/// - `DMNTK_DEFAULT_HOST` and `DMNTK_DEFAULT_PORT` constants.
///
fn get_server_address(opt_host: Option<String>, opt_port: Option<String>) -> String {
  // resolve IP address
  let mut host = DMNTK_DEFAULT_HOST.to_string();
  if let Ok(host_ip_address) = env::var(DMNTK_HOST_VARIABLE) {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address specified in environment variable {}: {}", DMNTK_HOST_VARIABLE, host_ip_address);
    }
  }
  if let Some(host_ip_address) = opt_host {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address given as command option: {}", host_ip_address);
    }
  }
  // resolve IP port
  let mut port: u16 = DMNTK_DEFAULT_PORT;
  if let Ok(p_str) = env::var(DMNTK_PORT_VARIABLE) {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified in environment variable {}: {}", DMNTK_PORT_VARIABLE, p_str);
    }
  }
  if let Some(p_str) = opt_port {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified as command option: {}", p_str);
    }
  }
  let server_address = format!("{host}:{port}");
  server_address
}

/// Checks if the specified IP address is correct.
///
/// This function may provide more detailed checks
/// when the [Ipv4Addr](std::net::Ipv4Addr)
/// and [Ipv6Addr](std::net::Ipv6Addr) stabilize.
fn is_valid_ip_address(ip: &str) -> bool {
  ip == "localhost" || ip.parse::<IpAddr>().is_ok()
}

/// Returns the root directory for loading workspaces.
fn get_root_dir(opt_dir: Option<String>) -> PathBuf {
  let current_dir_path = env::current_dir().expect("failed to retrieve current directory");
  if let Ok(s) = env::var(DMNTK_DIR_VARIABLE) {
    let dir_path = Path::new(&s);
    if dir_path.exists() && dir_path.is_dir() {
      return dir_path.into();
    } else {
      eprintln!("invalid directory specified in environment variable {}: {}", DMNTK_DIR_VARIABLE, s);
    }
  }
  if let Some(s) = opt_dir {
    let dir_path = Path::new(&s);
    if dir_path.exists() && dir_path.is_dir() {
      return dir_path.into();
    } else {
      eprintln!("invalid directory specified as command option: {}", s);
    }
  }
  current_dir_path
}
