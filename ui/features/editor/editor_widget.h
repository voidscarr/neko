#pragma once

#include <QWidget>

class EditorWidget : public QWidget {
  Q_OBJECT

public:
  EditorWidget(QWidget *parent = nullptr);
  ~EditorWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;
};
