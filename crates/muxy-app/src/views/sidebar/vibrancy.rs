use crate::model::AppModel;
use gpui::Hsla;

impl AppModel {
    pub(crate) fn sidebar_background(&self) -> Hsla {
        self.theme.bg
    }
}
