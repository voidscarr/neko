#include "tab_container_widget.h"
#include "tab_widget.h"
#include <QFrame>
#include <QPainter>
#include <ffi.rs.h>

TabContainerWidget::TabContainerWidget(rust::Box<TabManager> &tabManager,
                                       rust::Box<Config> &config,
                                       rust::Box<ThemeManager> &themeManager,
                                       QWidget *parent)
    : m_tabManager(tabManager), m_config(config), m_themeManager(themeManager) {
  setFixedHeight(40);
  setContentsMargins(0, 0, 0, 0);
  setAutoFillBackground(false);

  setFocusPolicy(Qt::FocusPolicy::NoFocus);

  m_hBoxLayout = new QHBoxLayout(this);
  m_hBoxLayout->setContentsMargins(10, 6, 10, 6);
  m_hBoxLayout->setSpacing(0);
  m_hBoxLayout->setAlignment(Qt::AlignLeft);

  const auto tabs = tabs_ffi(*m_tabManager);
  for (auto tab : tabs) {
    m_tabWidget = new TabWidget(tab.active, tab.dirty, tab.title.c_str(),
                                tab.path.c_str(), config, themeManager, this);
  }

  m_hBoxLayout->addWidget(m_tabWidget);
}

void TabContainerWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

  const auto current_theme_name = m_config->theme_name();
  auto theme = theme_by_name_ffi(*m_themeManager, current_theme_name);

  painter.fillRect(rect(), theme.background.c_str());
}
