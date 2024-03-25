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

//! # Derive macros

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ToErrorMessage)]
pub fn to_error_message(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl ToErrorMessage for #name {
      fn message(self) -> String {
        self.0
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(DmnElement)]
pub fn dmn_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl DmnElement for #name {
      fn namespace(&self) -> &str {
        &self.namespace
      }
      fn id(&self) -> &String {
        match &self.id {
          DmnId::Provided(id) => id,
          DmnId::Generated(id) => id,
        }
      }
      fn opt_id(&self) -> Option<&String> {
        match &self.id {
          DmnId::Provided(id) => Some(&id),
          DmnId::Generated(_) => None,
        }
      }
      fn description(&self) -> &Option<String> {
        &self.description
      }
      fn label(&self) -> &Option<String> {
        &self.label
      }
      fn extension_elements(&self) -> &Vec<ExtensionElement> {
        &self.extension_elements
      }
      fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
        &self.extension_attributes
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(NamedElement)]
pub fn named_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl NamedElement for #name {
      fn name(&self) -> &str {
        &self.name
      }
      fn feel_name(&self) -> &Name {
        &self.feel_name
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(BusinessContextElement)]
pub fn business_context_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl BusinessContextElement for #name {
      fn uri(&self) -> &Option<String> {
        &self.uri
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(Expression)]
pub fn expression(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl Expression for #name {
      fn type_ref(&self) -> &Option<String> {
        &self.type_ref
      }
    }
  };
  TokenStream::from(expanded)
}
