/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use js::rust::HandleObject;

use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::DomRoot;
use crate::dom::document::Document;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::node::Node;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct HTMLHtmlElement {
    htmlelement: HTMLElement,
}

#[allow(non_snake_case)]
impl HTMLHtmlElement {
    fn new_inherited(
        localName: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLHtmlElement {
        HTMLHtmlElement {
            htmlelement: HTMLElement::new_inherited(localName, prefix, document),
        }
    }

    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn new(
        localName: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<HTMLHtmlElement> {
        let n = Node::reflect_node_with_proto(
            Box::new(HTMLHtmlElement::new_inherited(localName, prefix, document)),
            document,
            proto,
            can_gc,
        );

        n.upcast::<Node>().set_weird_parser_insertion_mode();
        n
    }
}
