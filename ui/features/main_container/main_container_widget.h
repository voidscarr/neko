#pragma once

#include "../editor/editor_widget.h"
#include "../tabs/tab_container_widget.h"
#include <QVBoxLayout>
#include <QWidget>
#include <ffi.rs.h>

class MainContainerWidget : public QWidget {
  Q_OBJECT

public:
  MainContainerWidget(QWidget *parent = nullptr);
  ~MainContainerWidget() = default;

private:
  rust::Box<TabManager> m_tabManager;
  rust::Box<Config> m_config;
  rust::Box<ThemeManager> m_themeManager;
  rust::Box<Editor> m_editor;

  QVBoxLayout *m_vBoxLayout;
  EditorWidget *m_editorWidget;
  TabContainerWidget *m_tabContainerWidget;
};
