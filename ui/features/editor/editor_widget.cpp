#include "editor_widget.h"
#include <QPainter>

EditorWidget::EditorWidget(QWidget *parent) { setAutoFillBackground(false); }

void EditorWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

  // Background
  painter.fillRect(rect(), "black");

  // TODO: Add theming
  // TODO: Move this to class/add font metrics inst.
  // Text
  QFont font = QFont("IBM Plex Mono", 15.0);
  painter.setPen("white");
  painter.setFont(font);

  painter.drawText(rect().topLeft() + QPoint(0, font.pointSize()), "meow");
}
