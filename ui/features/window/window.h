#pragma once

#include "../editor/editor_widget.h"
#include <QMainWindow>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();

private:
  EditorWidget *m_editorWidget;
};
