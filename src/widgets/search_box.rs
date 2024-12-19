// Copyright 2024 the Cartero authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::glib;

mod imp {
    use std::sync::OnceLock;

    use glib::object::ObjectExt;
    use glib::subclass::{InitializingObject, Signal};
    use glib::types::StaticType;
    use gtk::gio::{ActionEntry, SimpleActionGroup};
    use gtk::glib;
    use gtk::prelude::{ActionMapExtManual, EditableExt, WidgetExt};
    use gtk::subclass::prelude::*;
    use gtk::{CompositeTemplate, TemplateChild};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/es/danirod/Cartero/search_box.ui")]
    pub struct SearchBox {
        #[template_child]
        search_content: TemplateChild<gtk::Entry>,

        #[template_child]
        search_previous: TemplateChild<gtk::Button>,

        #[template_child]
        search_next: TemplateChild<gtk::Button>,

        #[template_child]
        search_close: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SearchBox {
        const NAME: &'static str = "CarteroSearchBox";
        type Type = super::SearchBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SearchBox {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("previous").build(),
                    Signal::builder("next").build(),
                    Signal::builder("close").build(),
                    Signal::builder("search")
                        .param_types([String::static_type()])
                        .build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.init_actions();
        }
    }

    impl WidgetImpl for SearchBox {}

    impl BoxImpl for SearchBox {}

    #[gtk::template_callbacks]
    impl SearchBox {
        fn init_actions(&self) {
            let obj = self.obj();
            let action_previous = ActionEntry::builder("previous")
                .activate(glib::clone!(@weak obj => move |_, _, _| {
                    obj.emit_by_name::<()>("previous", &[]);
                }))
                .build();
            let action_next = ActionEntry::builder("next")
                .activate(glib::clone!(@weak obj => move |_, _, _| {
                    obj.emit_by_name::<()>("next", &[]);
                }))
                .build();
            let action_close = ActionEntry::builder("close")
                .activate(glib::clone!(@weak obj => move |_, _, _| {
                    obj.emit_by_name::<()>("close", &[]);
                }))
                .build();
            let group = SimpleActionGroup::new();
            group.add_action_entries([action_previous, action_next, action_close]);
            obj.insert_action_group("search", Some(&group));
        }

        #[template_callback]
        fn on_text_changed(&self, entry: &gtk::Entry) {
            let text = entry.text();
            let obj = self.obj();
            obj.emit_by_name::<()>("search", &[&text]);
        }

        #[template_callback]
        fn on_text_activate(&self) {
            let obj = self.obj();
            obj.emit_by_name::<()>("next", &[]);
        }
    }
}

glib::wrapper! {
    pub struct SearchBox(ObjectSubclass<imp::SearchBox>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable;
}
