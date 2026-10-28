#pragma once

#include <QWidget>
#include <ffi.rs.h>

class TabContainerWidget : public QWidget {
  Q_OBJECT

public:
  TabContainerWidget(rust::Box<Config> &config,
                     rust::Box<ThemeManager> &themeManager,
                     QWidget *parent = nullptr);
  ~TabContainerWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;

  rust::Box<Config> &m_config;
  rust::Box<ThemeManager> &m_themeManager;
};
