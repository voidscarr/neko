#include "tab_widget.h"
#include <QPainter>

TabWidget::TabWidget(rust::Box<Config> &config,
                     rust::Box<ThemeManager> &themeManager, QWidget *parent)
    : m_config(config), m_themeManager(themeManager) {
  setAutoFillBackground(false);
  setFocusPolicy(Qt::FocusPolicy::NoFocus);
}

void TabWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

  const auto current_theme_name = m_config->theme_name();
  auto theme = theme_by_name_ffi(*m_themeManager, current_theme_name);

  painter.fillRect(rect(), theme.background.c_str());
}
