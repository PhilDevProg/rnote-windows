mod imp {
    use crate::colorpicker::ColorPicker;
    use gtk4::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};
    use gtk4::{MenuButton, Popover, SpinButton, Switch, Image, ListBox};

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/penssidebar/shaperpage.ui")]
    pub struct ShaperPage {
        #[template_child]
        pub shaperstyle_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub shaperstyle_image: TemplateChild<Image>,
        #[template_child]
        pub shaperstyle_listbox: TemplateChild<ListBox>,
        #[template_child]
        pub shaperstyle_smooth_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub shaperstyle_rough_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub shapeconfig_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub shapeconfig_popover: TemplateChild<Popover>,
        #[template_child]
        pub roughconfig_roughness_spinbutton: TemplateChild<SpinButton>,
        #[template_child]
        pub roughconfig_bowing_spinbutton: TemplateChild<SpinButton>,
        #[template_child]
        pub roughconfig_curvestepcount_spinbutton: TemplateChild<SpinButton>,
        #[template_child]
        pub roughconfig_multistroke_switch: TemplateChild<Switch>,
        #[template_child]
        pub width_spinbutton: TemplateChild<SpinButton>,
        #[template_child]
        pub stroke_colorpicker: TemplateChild<ColorPicker>,
        #[template_child]
        pub fill_colorpicker: TemplateChild<ColorPicker>,
        #[template_child]
        pub shapetype_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub shapetype_image: TemplateChild<Image>,
        #[template_child]
        pub shapetype_listbox: TemplateChild<ListBox>,
        #[template_child]
        pub shapetype_line_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub shapetype_rectangle_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub shapetype_ellipse_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShaperPage {
        const NAME: &'static str = "ShaperPage";
        type Type = super::ShaperPage;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShaperPage {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for ShaperPage {}
}

use crate::{appwindow::RnoteAppWindow, colorpicker::ColorPicker};
use gtk4::{gdk, MenuButton, Popover, SpinButton, Switch, ListBox, Image};
use gtk4::{glib, glib::clone, prelude::*, subclass::prelude::*};
use rnote_compose::style::rough::RoughOptions;
use rnote_engine::pens::shaper::ShaperStyle;
use rnote_engine::utils::GdkRGBAHelpers;

glib::wrapper! {
    pub struct ShaperPage(ObjectSubclass<imp::ShaperPage>)
        @extends gtk4::Widget;
}

impl Default for ShaperPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ShaperPage {
    /// The default width
    pub const WIDTH_DEFAULT: f64 = 2.0;
    /// The min width
    pub const WIDTH_MIN: f64 = 0.1;
    /// The max width
    pub const WIDTH_MAX: f64 = 1000.0;

    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ShaperPage")
    }

    pub fn shaperstyle_menubutton(&self) -> MenuButton {
        self.imp()
            .shaperstyle_menubutton
            .get()
    }

    pub fn shaperstyle_image(&self) -> Image {
        self.imp().shaperstyle_image.get()
    }

    pub fn shaperstyle_listbox(&self) -> ListBox {
        self.imp().shaperstyle_listbox.get()
    }

    pub fn shaperstyle_smooth_row(&self) -> adw::ActionRow {
        self.imp()
            .shaperstyle_smooth_row
            .get()
    }

    pub fn shaperstyle_rough_row(&self) -> adw::ActionRow {
        self.imp()
            .shaperstyle_rough_row
            .get()
    }

    pub fn shapeconfig_menubutton(&self) -> MenuButton {
        imp::ShaperPage::from_instance(self)
            .shapeconfig_menubutton
            .get()
    }

    pub fn shapeconfig_popover(&self) -> Popover {
        imp::ShaperPage::from_instance(self)
            .shapeconfig_popover
            .get()
    }

    pub fn width_spinbutton(&self) -> SpinButton {
        imp::ShaperPage::from_instance(self).width_spinbutton.get()
    }

    pub fn roughconfig_roughness_spinbutton(&self) -> SpinButton {
        imp::ShaperPage::from_instance(self)
            .roughconfig_roughness_spinbutton
            .get()
    }

    pub fn roughconfig_bowing_spinbutton(&self) -> SpinButton {
        imp::ShaperPage::from_instance(self)
            .roughconfig_bowing_spinbutton
            .get()
    }

    pub fn roughconfig_curvestepcount_spinbutton(&self) -> SpinButton {
        imp::ShaperPage::from_instance(self)
            .roughconfig_curvestepcount_spinbutton
            .get()
    }

    pub fn roughconfig_multistroke_switch(&self) -> Switch {
        imp::ShaperPage::from_instance(self)
            .roughconfig_multistroke_switch
            .get()
    }

    pub fn stroke_colorpicker(&self) -> ColorPicker {
        imp::ShaperPage::from_instance(self)
            .stroke_colorpicker
            .get()
    }

    pub fn fill_colorpicker(&self) -> ColorPicker {
        imp::ShaperPage::from_instance(self).fill_colorpicker.get()
    }

    pub fn shapetype_menubutton(&self) -> MenuButton {
        self.imp()
            .shapetype_menubutton
            .get()
    }

    pub fn shapetype_image(&self) -> Image {
        self.imp().shapetype_image.get()
    }

    pub fn shapetype_listbox(&self) -> ListBox {
        self.imp().shapetype_listbox.get()
    }

    pub fn shapetype_line_row(&self) -> adw::ActionRow {
        self.imp()
            .shapetype_line_row
            .get()
    }

    pub fn shapetype_rectangle_row(&self) -> adw::ActionRow {
        self.imp()
            .shapetype_rectangle_row
            .get()
    }

    pub fn shapetype_ellipse_row(&self) -> adw::ActionRow {
        self.imp()
            .shapetype_ellipse_row
            .get()
    }

    pub fn init(&self, appwindow: &RnoteAppWindow) {
        // Width
        self.width_spinbutton().set_increments(0.1, 2.0);
        self.width_spinbutton()
            .set_range(Self::WIDTH_MIN, Self::WIDTH_MAX);
        self.width_spinbutton().set_value(Self::WIDTH_DEFAULT);

        self.width_spinbutton().connect_value_changed(
            clone!(@weak appwindow => move |width_spinbutton| {
                let shaper_style = appwindow.canvas().engine().borrow_mut().penholder.shaper.style;

                match shaper_style {
                    ShaperStyle::Smooth => appwindow.canvas().engine().borrow_mut().penholder.shaper.smooth_options.width = width_spinbutton.value(),
                    ShaperStyle::Rough => appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.stroke_width = width_spinbutton.value(),
                }
            }),
        );

        // Stroke color
        self.stroke_colorpicker().connect_notify_local(
            Some("current-color"),
            clone!(@weak appwindow => move |stroke_colorpicker, _paramspec| {
                let color = stroke_colorpicker.property::<gdk::RGBA>("current-color").into_compose_color();
                let shaper_style = appwindow.canvas().engine().borrow_mut().penholder.shaper.style;

                match shaper_style {
                    ShaperStyle::Smooth => appwindow.canvas().engine().borrow_mut().penholder.shaper.smooth_options.stroke_color = Some(color),
                    ShaperStyle::Rough => appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.stroke_color= Some(color),
                }
            }),
        );

        // Fill color
        self.fill_colorpicker().connect_notify_local(
            Some("current-color"),
            clone!(@weak appwindow => move |fill_colorpicker, _paramspec| {
                let color = fill_colorpicker.property::<gdk::RGBA>("current-color").into_compose_color();
                let shaper_style = appwindow.canvas().engine().borrow_mut().penholder.shaper.style;

                match shaper_style {
                    ShaperStyle::Smooth => appwindow.canvas().engine().borrow_mut().penholder.shaper.smooth_options.fill_color = Some(color),
                    ShaperStyle::Rough => appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.fill_color= Some(color),
                }
            }),
        );

        // Roughness
        self.imp()
            .roughconfig_roughness_spinbutton
            .get()
            .set_increments(0.1, 2.0);
        self.imp()
            .roughconfig_roughness_spinbutton
            .get()
            .set_range(RoughOptions::ROUGHNESS_MIN, RoughOptions::ROUGHNESS_MAX);
        self.imp()
            .roughconfig_roughness_spinbutton
            .get()
            .set_value(RoughOptions::ROUGHNESS_DEFAULT);

        self.imp().roughconfig_roughness_spinbutton.get().connect_value_changed(
            clone!(@weak appwindow => move |roughconfig_roughness_spinbutton| {
                appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.roughness = roughconfig_roughness_spinbutton.value();
            }),
        );

        // Bowing
        self.imp()
            .roughconfig_bowing_spinbutton
            .get()
            .set_increments(0.1, 2.0);
        self.imp()
            .roughconfig_bowing_spinbutton
            .get()
            .set_range(RoughOptions::BOWING_MIN, RoughOptions::BOWING_MAX);
        self.imp()
            .roughconfig_bowing_spinbutton
            .get()
            .set_value(RoughOptions::BOWING_DEFAULT);

        self.imp().roughconfig_bowing_spinbutton.get().connect_value_changed(
            clone!(@weak appwindow => move |roughconfig_bowing_spinbutton| {
                appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.bowing = roughconfig_bowing_spinbutton.value();
            }),
        );

        // Curve stepcount
        self.imp()
            .roughconfig_curvestepcount_spinbutton
            .get()
            .set_increments(1.0, 2.0);
        self.imp()
            .roughconfig_curvestepcount_spinbutton
            .get()
            .set_range(
                RoughOptions::CURVESTEPCOUNT_MIN,
                RoughOptions::CURVESTEPCOUNT_MAX,
            );
        self.imp()
            .roughconfig_curvestepcount_spinbutton
            .get()
            .set_value(RoughOptions::CURVESTEPCOUNT_DEFAULT);

        self.imp().roughconfig_curvestepcount_spinbutton.get().connect_value_changed(
            clone!(@weak appwindow => move |roughconfig_curvestepcount_spinbutton| {
                appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.curve_stepcount = roughconfig_curvestepcount_spinbutton.value();
            }),
        );

        // Multistroke
        self.imp().roughconfig_multistroke_switch.get().connect_state_notify(clone!(@weak appwindow => move |roughconfig_multistroke_switch| {
            appwindow.canvas().engine().borrow_mut().penholder.shaper.rough_options.disable_multistroke = roughconfig_multistroke_switch.state();
        }));

        // Smooth / Rough shaper style
        self.shaperstyle_listbox().connect_row_selected(
            clone!(@weak self as shaperpage, @weak appwindow => move |_shaperstyle_listbox, selected_row| {
                if let Some(selected_row) = selected_row.map(|selected_row| {selected_row.downcast_ref::<adw::ActionRow>().unwrap()}) {
                    match selected_row.index() {
                        // Smooth
                        0 => {
                            adw::prelude::ActionGroupExt::activate_action(&appwindow, "shaper-style", Some(&"smooth".to_variant()));
                        }
                        // Rough
                        1 => {
                            adw::prelude::ActionGroupExt::activate_action(&appwindow, "shaper-style", Some(&"rough".to_variant()));
                        }
                        _ => {}
                    }
                }
            }),
        );

        // shape type
        self.shapetype_listbox().connect_row_selected(
            clone!(@weak self as shaperpage, @weak appwindow => move |_shapetype_listbox, selected_row| {
                if let Some(selected_row) = selected_row.map(|selected_row| {selected_row.downcast_ref::<adw::ActionRow>().unwrap()}) {
                    match selected_row.index() {
                        // Line
                        0 => {
                            adw::prelude::ActionGroupExt::activate_action(&appwindow, "shape-type", Some(&"line".to_variant()));
                        }
                        // Rectangle
                        1 => {
                            adw::prelude::ActionGroupExt::activate_action(&appwindow, "shape-type", Some(&"rectangle".to_variant()));
                        }
                        // Ellipse
                        2 => {
                            adw::prelude::ActionGroupExt::activate_action(&appwindow, "shape-type", Some(&"ellipse".to_variant()));
                        }
                        _ => {}
                    }
                }
            }),
        );
    }
}