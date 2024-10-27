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

use glib::{object::ObjectExt, subclass::types::ObjectSubclassIsExt};

use crate::entities::{EndpointData, RequestExportType};

use super::{BaseExportPaneExt, CurlService};

mod imp {
    use std::cell::RefCell;
    use std::sync::OnceLock;

    use adw::subclass::bin::BinImpl;
    use glib::subclass::{InitializingObject, Signal};
    use glib::Properties;
    use gtk::subclass::prelude::*;
    use gtk::{gio::SettingsBindFlags, CompositeTemplate};
    use gtk::{prelude::*, WrapMode};
    use sourceview5::{prelude::*, LanguageManager};
    use sourceview5::{Buffer, StyleSchemeManager, View};

    use crate::app::CarteroApplication;
    use crate::widgets::{BaseExportPane, BaseExportPaneImpl, ExportType};

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::CurlExportPane)]
    #[template(resource = "/es/danirod/Cartero/curl_export_pane.ui")]
    pub struct CurlExportPane {
        #[template_child]
        view: TemplateChild<View>,

        #[template_child]
        buffer: TemplateChild<Buffer>,

        #[property(get = Self::format, set = Self::set_format, builder(ExportType::default()))]
        _format: RefCell<ExportType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CurlExportPane {
        const NAME: &'static str = "CarteroCurlExportPane";

        type Type = super::CurlExportPane;
        type ParentType = BaseExportPane;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CurlExportPane {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("changed").build()])
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.init_settings();
            self.init_source_view_style();
        }
    }

    impl WidgetImpl for CurlExportPane {}

    impl BinImpl for CurlExportPane {}

    impl BaseExportPaneImpl for CurlExportPane {}

    impl CurlExportPane {
        #[allow(unused)]
        pub(super) fn buffer_content(&self) -> Vec<u8> {
            let (start, end) = self.buffer.bounds();
            let text = self.buffer.text(&start, &end, true);
            Vec::from(text)
        }

        #[allow(unused)]
        pub(super) fn set_buffer_content(&self, payload: &[u8]) {
            let body = String::from_utf8_lossy(payload);
            self.buffer.set_text(&body);
        }

        fn format(&self) -> ExportType {
            // Maybe in a future we're gonna add axios and others here, not sure
            // if that thing should be in another widget or this could could be
            // reutilized for that purpose, since we're just gonna use anyways a
            // textbox, [axios export issue](https://github.com/danirod/cartero/issues/64)
            ExportType::Curl
        }

        fn set_format(&self, format: ExportType) {
            let manager = LanguageManager::default();
            let language = match format {
                ExportType::Curl => manager.language("shell"),
                _ => None,
            };

            self.buffer.set_language(language.as_ref());
        }

        fn init_settings(&self) {
            let app = CarteroApplication::get();
            let settings = app.settings();

            settings
                .bind("body-wrap", &*self.view, "wrap-mode")
                .flags(SettingsBindFlags::GET)
                .mapping(|variant, _| {
                    let enabled = variant.get::<bool>().expect("The variant is not a boolean");
                    let mode = match enabled {
                        true => WrapMode::Word,
                        false => WrapMode::None,
                    };
                    Some(mode.to_value())
                })
                .build();

            settings
                .bind("show-line-numbers", &*self.view, "show-line-numbers")
                .flags(SettingsBindFlags::GET)
                .build();
            settings
                .bind("auto-indent", &*self.view, "auto-indent")
                .flags(SettingsBindFlags::GET)
                .build();
            settings
                .bind("indent-style", &*self.view, "insert-spaces-instead-of-tabs")
                .flags(SettingsBindFlags::GET)
                .mapping(|variant, _| {
                    let mode = variant
                        .get::<String>()
                        .expect("The variant is not a string");
                    let use_spaces = mode == "spaces";
                    Some(use_spaces.to_value())
                })
                .build();
            settings
                .bind("tab-width", &*self.view, "tab-width")
                .flags(SettingsBindFlags::GET)
                .mapping(|variant, _| {
                    let width = variant.get::<String>().unwrap_or("4".into());
                    let value = width.parse::<i32>().unwrap_or(4);
                    Some(value.to_value())
                })
                .build();
            settings
                .bind("tab-width", &*self.view, "indent-width")
                .flags(SettingsBindFlags::GET)
                .mapping(|variant, _| {
                    let width = variant.get::<String>().unwrap_or("4".into());
                    let value = width.parse::<i32>().unwrap_or(4);
                    Some(value.to_value())
                })
                .build();
        }

        fn update_source_view_style(&self) {
            let dark_mode = adw::StyleManager::default().is_dark();
            let color_theme = if dark_mode { "Adwaita-dark" } else { "Adwaita" };
            let theme = StyleSchemeManager::default().scheme(color_theme);

            match theme {
                Some(theme) => {
                    self.buffer.set_style_scheme(Some(&theme));
                    self.buffer.set_highlight_syntax(true);
                }
                None => {
                    self.buffer.set_highlight_syntax(false);
                }
            }
        }

        fn init_source_view_style(&self) {
            self.update_source_view_style();
            adw::StyleManager::default().connect_dark_notify(
                glib::clone!(@weak self as panel => move |_| {
                    panel.update_source_view_style();
                }),
            );
        }
    }
}

glib::wrapper! {
    pub struct CurlExportPane(ObjectSubclass<imp::CurlExportPane>)
        @extends gtk::Widget, adw::Bin, super::BaseExportPane,
        @implements gtk::Accessible, gtk::Buildable;
}

impl CurlExportPane {
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "changed",
            true,
            glib::closure_local!(|ref pane| {
                f(pane);
            }),
        )
    }
}

impl BaseExportPaneExt for CurlExportPane {
    fn request_export_type(&self) -> RequestExportType {
        // TODO: Maybe we could extract url and others from the service which is
        // gonna generate curl output or idk, like reparse the generated content
        // to extract its data and regenerate a new endpoint data.
        match self.format() {
            super::ExportType::Curl => RequestExportType::Curl(EndpointData::default()),
            _ => RequestExportType::None,
        }
    }

    fn set_request_export_type(&self, req_export_type: &RequestExportType) {
        if let RequestExportType::Curl(data) = req_export_type {
            let service = CurlService::new(data.clone());
            let imp = self.imp();

            imp.set_buffer_content(service.generate().as_bytes());
        }
    }
}
