#include "tab_container_widget.h"
#include <QFrame>
#include <QPainter>

TabContainerWidget::TabContainerWidget(rust::Box<Config> &config,
                                       rust::Box<ThemeManager> &themeManager,
                                       QWidget *parent)
    : m_config(config), m_themeManager(themeManager) {
  setFixedHeight(50);
  setContentsMargins(0, 0, 0, 0);
  setAutoFillBackground(false);

  setFocusPolicy(Qt::FocusPolicy::NoFocus);
}

void TabContainerWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

  const auto current_theme_name = m_config->theme_name();
  auto theme = theme_by_name_ffi(*m_themeManager, current_theme_name);

  painter.fillRect(rect(), theme.background.c_str());
}
