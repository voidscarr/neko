#include "window.h"
#include "../tabs/tab_container_widget.h"

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  m_mainContainerWidget = new MainContainerWidget(this);
  setCentralWidget(m_mainContainerWidget);
}

MainWindow::~MainWindow() {}
