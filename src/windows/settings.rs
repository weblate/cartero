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

use adw::prelude::AdwDialogExt;
use glib::{object::IsA, Object};
use gtk::gio;

mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{template_callbacks, CompositeTemplate, TemplateChild};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/es/danirod/Cartero/settings_dialog.ui")]
    pub struct SettingsDialog {
        #[template_child]
        option_validate_tls: TemplateChild<adw::SwitchRow>,

        #[template_child]
        option_follow_redirects: TemplateChild<adw::ExpanderRow>,

        #[template_child]
        option_maximum_redirects: TemplateChild<adw::SpinRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsDialog {
        const NAME: &'static str = "CarteroSettingsDialog";
        type Type = super::SettingsDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SettingsDialog {}

    impl WidgetImpl for SettingsDialog {}

    impl WindowImpl for SettingsDialog {}

    impl AdwDialogImpl for SettingsDialog {}

    impl PreferencesDialogImpl for SettingsDialog {}

    #[template_callbacks]
    impl SettingsDialog {}
}

glib::wrapper! {
    pub struct SettingsDialog(ObjectSubclass<imp::SettingsDialog>)
        @extends gtk::Widget, gtk::Window, adw::Dialog, adw::PreferencesDialog,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

impl SettingsDialog {
    pub fn present_for_window(win: &impl IsA<gtk::Widget>) {
        let dialog: Self = Object::builder().build();
        dialog.present(win);
    }
}
