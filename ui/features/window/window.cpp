#include "window.h"

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  m_editorWidget = new EditorWidget(this);
  setCentralWidget(m_editorWidget);
}

MainWindow::~MainWindow() {}
