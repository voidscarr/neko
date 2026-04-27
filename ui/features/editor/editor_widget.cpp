#include "editor_widget.h"
#include <QKeyEvent>
#include <QPainter>
#include <ffi.rs.h>

EditorWidget::EditorWidget(QWidget *parent)
    : m_font(QFont("IBM Plex Mono", 15.0)),
      m_fontMetrics(QFontMetricsF(m_font)), m_buffer(new_empty_buffer()),
      QWidget(parent) {
  setAutoFillBackground(false);
  setFocusPolicy(Qt::FocusPolicy::StrongFocus);
  focusWidget();
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
                   m_buffer->content(0, m_buffer->len()).c_str());
}

void EditorWidget::keyPressEvent(QKeyEvent *event) {
  m_buffer->insert_char(event->text().toStdString().c_str());
  repaint();
}
