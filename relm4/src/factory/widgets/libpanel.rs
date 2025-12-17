use panel::prelude::*;

use crate::factory::FactoryView;

impl FactoryView for panel::Paned {
    type Children = gtk::Widget;
    type ReturnedWidget = gtk::Widget;
    type Position = ();

    fn factory_remove(&self, widget: &gtk::Widget) {
        self.remove(widget);
    }

    fn factory_append(&self, widget: impl AsRef<gtk::Widget>, _: &()) -> gtk::Widget {
        self.append(widget.as_ref());
        widget.as_ref().clone()
    }

    fn factory_prepend(&self, widget: impl AsRef<gtk::Widget>, _: &()) -> gtk::Widget {
        self.prepend(widget.as_ref());
        widget.as_ref().clone()
    }

    fn factory_insert_after(
        &self,
        widget: impl AsRef<gtk::Widget>,
        _: &(),
        other: &gtk::Widget,
    ) -> gtk::Widget {
        self.insert_after(widget.as_ref(), other);
        widget.as_ref().clone()
    }

    fn returned_widget_to_child(root_child: &gtk::Widget) -> gtk::Widget {
        root_child.clone()
    }

    fn factory_move_after(&self, widget: &gtk::Widget, other: &gtk::Widget) {
        self.insert_after(widget, other);
    }

    fn factory_move_start(&self, widget: &gtk::Widget) {
        self.insert(0, widget);
    }
}

impl FactoryView for panel::DocumentWorkspace {
    type Children = gtk::Widget;
    type ReturnedWidget = panel::Widget;
    type Position = panel::Area;

    fn factory_remove(&self, widget: &Self::ReturnedWidget) {
        widget.close();
    }

    fn factory_append(
        &self,
        child: impl AsRef<Self::Children>,
        position: &Self::Position,
    ) -> Self::ReturnedWidget {
        let widget = panel::Widget::new();
        widget.set_child(Some(child.as_ref()));
        let w_pos = panel::Position::builder().area(*position).build();
        self.add_widget(&widget, Some(&w_pos));
        widget
    }

    fn factory_prepend(
        &self,
        widget: impl AsRef<Self::Children>,
        position: &Self::Position,
    ) -> Self::ReturnedWidget {
        self.factory_append(widget, position)
    }

    fn factory_insert_after(
        &self,
        widget: impl AsRef<Self::Children>,
        position: &Self::Position,
        _other: &Self::ReturnedWidget,
    ) -> Self::ReturnedWidget {
        self.factory_append(widget, position)
    }

    fn returned_widget_to_child(root_child: &Self::ReturnedWidget) -> Self::Children {
        root_child
            .child()
            .expect("That panel widget has no child????")
    }

    fn factory_move_after(&self, _widget: &Self::ReturnedWidget, _other: &Self::ReturnedWidget) {}

    fn factory_move_start(&self, _widget: &Self::ReturnedWidget) {}
}
