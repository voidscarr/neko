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

  // TODO: Implement line wrapping in the future (option toggle)
  painter.drawText(rect(), m_buffer->content_slice(0, m_buffer->len()).c_str());
}

void EditorWidget::keyPressEvent(QKeyEvent *event) {
  switch (event->key()) {
  case Qt::Key_Enter:
  case Qt::Key_Return:
    m_buffer->insert_char("\n");
    break;
  case Qt::Key_Tab:
    // TODO: Make this configurable for \t or 2/4/8+ spaces
    m_buffer->insert_char("    ");
    break;
  case Qt::Key_Backspace:
    m_buffer->remove_char(m_buffer->len() - 1, m_buffer->len());
    break;
  default:
    m_buffer->insert_char(event->text().toStdString().c_str());
    break;
  }

  repaint();
}
