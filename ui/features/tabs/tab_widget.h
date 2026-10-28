#pragma once

#include <QWidget>
#include <ffi.rs.h>

class TabWidget : public QWidget {
  Q_OBJECT

public:
  TabWidget(rust::Box<Config> &config, rust::Box<ThemeManager> &themeManager,
            QWidget *parent = nullptr);
  ~TabWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;

  rust::Box<Config> &m_config;
  rust::Box<ThemeManager> &m_themeManager;
};
