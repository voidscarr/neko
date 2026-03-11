#include "editor_widget.h"
#include <QPainter>

EditorWidget::EditorWidget(QWidget *parent)
    : m_font(QFont("IBM Plex Mono", 15.0)),
      m_fontMetrics(QFontMetricsF(m_font)), QWidget(parent) {
  setAutoFillBackground(false);
}

void EditorWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

  // Background
  painter.fillRect(rect(), "black");

  // Text
  // TODO: Add theming
  painter.setPen("white");
  painter.setFont(m_font);

  painter.drawText(rect().topLeft() + QPoint(0, m_fontMetrics.height()),
                   "meow");
}
