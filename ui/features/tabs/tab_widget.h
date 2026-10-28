#pragma once

#include <QHBoxLayout>
#include <QWidget>
#include <ffi.rs.h>

class TabWidget : public QWidget {
  Q_OBJECT

public:
  TabWidget(bool active, bool dirty, std::string title, std::string path,
            rust::Box<Config> &config, rust::Box<ThemeManager> &themeManager,
            QWidget *parent = nullptr);
  ~TabWidget() = default;

private:
  rust::Box<Config> &m_config;
  rust::Box<ThemeManager> &m_themeManager;

  QHBoxLayout *m_hBoxLayout;
};
