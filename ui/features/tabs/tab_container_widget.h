#pragma once

#include "tab_widget.h"
#include <QHBoxLayout>
#include <QWidget>
#include <ffi.rs.h>

class TabContainerWidget : public QWidget {
  Q_OBJECT

public:
  TabContainerWidget(rust::Box<TabManager> &tabManager,
                     rust::Box<Config> &config,
                     rust::Box<ThemeManager> &themeManager,
                     QWidget *parent = nullptr);
  ~TabContainerWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;

  rust::Box<TabManager> &m_tabManager;
  rust::Box<Config> &m_config;
  rust::Box<ThemeManager> &m_themeManager;

  QHBoxLayout *m_hBoxLayout;
  TabWidget *m_tabWidget;
};
