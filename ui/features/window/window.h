#ifndef WINDOW_H
#define WINDOW_H

#include <QMainWindow>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();
};

#endif // WINDOW_H
