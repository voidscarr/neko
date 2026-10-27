#include "editor_widget.h"
#include <QApplication>
#include <QClipboard>
#include <QKeyEvent>
#include <QPainter>
#include <ffi.rs.h>

EditorWidget::EditorWidget(QWidget *parent)
    : m_font(QFont("IBM Plex Mono", 15.0)),
      m_fontMetrics(QFontMetricsF(m_font)), m_editor(new_editor()),
      m_config(new_config()), QAbstractScrollArea(parent) {
  m_config->load_config();
  m_font.setPointSizeF(m_config->font_size());

  setAutoFillBackground(false);
  setFocusPolicy(Qt::FocusPolicy::StrongFocus);
  focusWidget();
}

void EditorWidget::resizeEvent(QResizeEvent *event) { performResize(); }

void EditorWidget::wheelEvent(QWheelEvent *event) {
  horizontalScrollBar()->setValue(horizontalScrollBar()->value() +
                                  (event->angleDelta().x() / 8));
  verticalScrollBar()->setValue(verticalScrollBar()->value() +
                                (event->angleDelta().y() / 8));
}

QSize EditorWidget::calculateResize() {
  const auto viewportHeight = viewport()->height();
  const auto viewportWidth = viewport()->width();

  const auto rows = m_editor->num_rows();
  std::size_t maxChars = 0;

  for (int i = 0; i < rows; i++) {
    maxChars = qMax(maxChars, m_editor->line_len(i));
  }

  int contentHeight = rows * m_fontMetrics.height();
  int contentWidth = maxChars * m_fontMetrics.averageCharWidth();

  return QSize(contentWidth - viewportWidth + VIEWPORT_PADDING,
               contentHeight - viewportHeight + VIEWPORT_PADDING);
}

void EditorWidget::performResize() {
  const auto size = calculateResize();

  horizontalScrollBar()->setRange(0, size.width());
  verticalScrollBar()->setRange(0, size.height());
  updateGeometry();
}

void EditorWidget::scrollToCursor() {
  const auto cursorCol = m_editor->cursor_col();
  const auto cursorRow = m_editor->cursor_row();

  const auto charWidth = m_fontMetrics.averageCharWidth();
  const auto lineHeight = m_fontMetrics.height();

  const auto cursorX = cursorCol * charWidth;
  const auto cursorY = cursorRow * lineHeight;

  if (cursorX >
      horizontalScrollBar()->value() + viewport()->width() - VIEWPORT_PADDING) {
    horizontalScrollBar()->setValue(cursorX + VIEWPORT_PADDING -
                                    viewport()->width());
  } else if (cursorX < horizontalScrollBar()->value() + VIEWPORT_PADDING) {
    horizontalScrollBar()->setValue(cursorX - VIEWPORT_PADDING);
  }

  if (cursorY >
      verticalScrollBar()->value() + viewport()->height() - VIEWPORT_PADDING) {
    verticalScrollBar()->setValue(cursorY + VIEWPORT_PADDING -
                                  viewport()->height());
  } else if (cursorY < verticalScrollBar()->value() + VIEWPORT_PADDING) {
    verticalScrollBar()->setValue(cursorY - VIEWPORT_PADDING);
  }
}

void EditorWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(viewport());
  painter.setClipRect(viewport()->rect());
  painter.setClipping(true);

  // Background
  painter.fillRect(rect(), "black");

  // TODO: Add theming
  // Text
  painter.setPen("#ffffff");
  painter.setBrush("#ffffff");
  painter.setFont(m_font);

  const auto fontMetrics = QFontMetricsF(m_font);
  const auto charWidth = fontMetrics.averageCharWidth();
  const auto lineHeight = fontMetrics.height();

  // TODO: Implement line wrapping option toggle
  int startingLine = verticalScrollBar()->value() / lineHeight;
  int endingLine =
      (verticalScrollBar()->value() + viewport()->height()) / lineHeight;

  for (int i = startingLine;
       i < m_editor->num_rows() && i < endingLine + LINE_DRAWING_BUFFER; i++) {
    const auto x = -horizontalScrollBar()->value();
    const auto y = ((i + 1 - startingLine) * fontMetrics.height() -
                    (fontMetrics.height() / 4));

    painter.drawText(x, y, m_editor->line(i).c_str());
  }

  // Selection
  if (m_editor->selection_active()) {
    const auto selStart = selection_start_ffi(m_editor);
    const auto selEnd = selection_end_ffi(m_editor);

    painter.setPen(Qt::NoPen);
    painter.setBrush(QColor("#40a162a1"));

    for (int i = startingLine;
         i < endingLine + LINE_DRAWING_BUFFER &&
         i < startingLine + (selEnd.row - selStart.row + 1) &&
         i < m_editor->num_rows();
         i++) {
      const auto lineLength = m_editor->line_len(i);
      const auto x = -horizontalScrollBar()->value();
      const auto y = (lineHeight * (i - startingLine));

      painter.drawRect(
          QRectF(QPointF(x, y), QSizeF(charWidth * lineLength, lineHeight)));
    }
  }

  // Cursor
  const auto cursorX =
      (m_editor->cursor_col() * charWidth) - horizontalScrollBar()->value();
  const auto cursorY = (m_editor->cursor_row() - startingLine) * lineHeight;

  painter.setPen("#a162a1");
  painter.setBrush(QColor("#a162a1"));
  painter.drawRect(
      QRectF(QPointF(cursorX, cursorY), QSizeF(CURSOR_WIDTH, lineHeight)));
}

void EditorWidget::keyPressEvent(QKeyEvent *event) {
  const auto ctrlActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::ControlModifier);
  const auto cmdActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::MetaModifier);
  const auto altActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::AltModifier);
  const auto shiftActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::ShiftModifier);

  bool selectionActive = m_editor->selection_active();

  if (ctrlActive) {
    switch (event->key()) {
    case Qt::Key_A:
      m_editor->select_all();

      performResize();
      viewport()->repaint();
      return;
    case Qt::Key_V: {
      const std::string clipboardText =
          QApplication::clipboard()->text().toStdString();

      m_editor->delete_selection_and_clear();

      const auto c = CursorFFI{
          .row = m_editor->cursor_row(),
          .column = m_editor->cursor_col(),
          .sticky_column = m_editor->cursor_col(),
      };

      m_editor->insert_char(pos_to_index_ffi(*m_editor, c),
                            clipboardText.c_str());

      performResize();
      scrollToCursor();
      viewport()->repaint();
      return;
    }
    case Qt::Key_C: {
      if (selectionActive) {
        const std::string text = m_editor->selected_text().c_str();
        QApplication::clipboard()->setText(text.c_str());
      }

      performResize();
      viewport()->repaint();
      return;
    }
    case Qt::Key_X: {
      if (selectionActive) {
        const std::string text = m_editor->selected_text().c_str();
        QApplication::clipboard()->setText(text.c_str());
      }
      m_editor->delete_selection_and_clear();

      performResize();
      scrollToCursor();
      viewport()->repaint();
      return;
    }
    case Qt::Key_Equal:
      increaseFontSize();
      return;
    case Qt::Key_Minus:
      decreaseFontSize();
      return;
    }
  }

  switch (event->key()) {
  case Qt::Key_Enter:
  case Qt::Key_Return: {
    const auto c = makeCursorFFI();

    m_editor->insert_char(pos_to_index_ffi(*m_editor, c), "\n");
    break;
  }
  case Qt::Key_Tab: {
    const auto c = makeCursorFFI();

    // TODO: Make this configurable for \t or 2/4/8+ spaces
    m_editor->insert_char(pos_to_index_ffi(*m_editor, c), "    ");
    break;
  }
  case Qt::Key_Backspace:
    if (!selectionActive) {
      m_editor->remove_char(m_editor->content_len() - 1,
                            m_editor->content_len());
    } else {
      m_editor->delete_selection_and_clear();
    }

    break;

  case Qt::Key_Left:
    m_editor->move_left(1, !shiftActive);
    break;
  case Qt::Key_Right:
    m_editor->move_right(1, !shiftActive);
    break;
  case Qt::Key_Down:
    m_editor->move_down(1, !shiftActive);
    break;
  case Qt::Key_Up:
    m_editor->move_up(1, !shiftActive);
    break;

  default: {
    // Skip char entry if a modifier is pressed by itself.
    if (ctrlActive || cmdActive || altActive) {
      return;
    }

    // Skip char entry if ESC or Shift is pressed.
    if (event->key() == Qt::Key_Escape || event->key() == Qt::Key_Shift) {
      return;
    }

    const auto c = makeCursorFFI();
    m_editor->insert_char(pos_to_index_ffi(*m_editor, c),
                          event->text().toStdString().c_str());
    break;
  }
  }

  performResize();
  scrollToCursor();
  viewport()->repaint();
}

void EditorWidget::increaseFontSize() {
  m_config->set_font_size(m_font.pointSizeF() + FONT_SIZE_INCREMENT);
  m_font.setPointSizeF(m_font.pointSizeF() + FONT_SIZE_INCREMENT);
  m_fontMetrics = QFontMetricsF(m_font);
  VIEWPORT_PADDING = qMax(100.0, m_font.pointSizeF() * 3);

  viewport()->repaint();
  viewport()->updateGeometry();
}

void EditorWidget::decreaseFontSize() {
  m_config->set_font_size(m_font.pointSizeF() + FONT_SIZE_INCREMENT);

  if (m_font.pointSizeF() < 8) {
    m_font.setPointSizeF(8.0);
  } else {
    m_font.setPointSizeF(m_font.pointSizeF() - FONT_SIZE_INCREMENT);
  }

  VIEWPORT_PADDING = qMax(100.0, m_font.pointSizeF() * 3);
  m_fontMetrics = QFontMetricsF(m_font);

  viewport()->repaint();
  viewport()->updateGeometry();
}

CursorFFI EditorWidget::makeCursorFFI() {
  const auto c = CursorFFI{
      .row = m_editor->cursor_row(),
      .column = m_editor->cursor_col(),
      .sticky_column = m_editor->cursor_col(),
  };

  return c;
}
