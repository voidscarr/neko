#include "editor_widget.h"
#include <QApplication>
#include <QClipboard>
#include <QKeyEvent>
#include <QPainter>
#include <ffi.rs.h>

EditorWidget::EditorWidget(QWidget *parent)
    : m_font(QFont("IBM Plex Mono", 15.0)),
      m_fontMetrics(QFontMetricsF(m_font)), m_editor(new_editor()),
      QWidget(parent) {
  setAutoFillBackground(false);
  setFocusPolicy(Qt::FocusPolicy::StrongFocus);
  focusWidget();
}

void EditorWidget::paintEvent(QPaintEvent *event) {
  QPainter painter(this);

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
  for (int i = 0; i < m_editor->num_rows(); i++) {
    painter.drawText(
        0, (i + 1) * fontMetrics.height() - (fontMetrics.height() / 4),
        m_editor->line(i).c_str());
  }

  // Selection
  if (m_editor->selection_active()) {
    const auto selStart = selection_start_ffi(m_editor);
    const auto selEnd = selection_end_ffi(m_editor);
    painter.setPen("#40a162a1");
    painter.setBrush(QColor("#40a162a1"));

    // First line
    painter.drawRect(QRectF(QPointF(), QPointF()));

    // Middle lines
    for (int i = 0; i < selEnd.row - selStart.row; i++) {
      const auto lineLength = m_editor->line_len(i);

      painter.drawRect(QRectF(QPointF(0, (lineHeight * i) - 1),
                              QSizeF(charWidth * lineLength, lineHeight)));
    }
  }

  // Cursor
  const auto cursorX = m_editor->cursor_col() * charWidth;
  const auto cursorY = m_editor->cursor_row() * lineHeight;

  painter.setPen("#a162a1");
  painter.setBrush(QColor("#a162a1"));
  painter.drawRect(
      QRectF(QPointF(cursorX, cursorY), QSizeF(CURSOR_WIDTH, lineHeight)));
}

void EditorWidget::keyPressEvent(QKeyEvent *event) {
  auto ctrlActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::ControlModifier);
  auto cmdActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::MetaModifier);
  auto altActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::AltModifier);

  bool selectionActive = m_editor->selection_active();

  if (ctrlActive) {
    switch (event->key()) {
    case Qt::Key_A:
      m_editor->select_all();

      repaint();
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

      repaint();
      return;
    }
    case Qt::Key_C: {
      if (selectionActive) {
        const std::string text = m_editor->selected_text().c_str();
        QApplication::clipboard()->setText(text.c_str());
      }

      repaint();
      return;
    }
    case Qt::Key_X: {
      if (selectionActive) {
        const std::string text = m_editor->selected_text().c_str();
        QApplication::clipboard()->setText(text.c_str());
      }
      m_editor->delete_selection_and_clear();

      repaint();
      return;
    }
    }
  }

  switch (event->key()) {
  case Qt::Key_Enter:
  case Qt::Key_Return: {
    const auto c =
        makeCursorFFI(m_editor->cursor_row(), m_editor->cursor_row());

    m_editor->insert_char(pos_to_index_ffi(*m_editor, c), "\n");
    break;
  }
  case Qt::Key_Tab: {
    const auto c =
        makeCursorFFI(m_editor->cursor_row(), m_editor->cursor_row());

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
    // TODO: Deselect
    m_editor->move_left(1);
    break;
  case Qt::Key_Right:
    // TODO: Deselect
    m_editor->move_right(1);
    break;
  case Qt::Key_Down:
    // TODO: Deselect
    m_editor->move_down(1);
    break;
  case Qt::Key_Up:
    // TODO: Deselect
    m_editor->move_up(1);
    break;

  default: {
    // Skip char entry if a modifier is pressed by itself.
    if (ctrlActive || cmdActive || altActive) {
      return;
    }

    const auto c =
        makeCursorFFI(m_editor->cursor_row(), m_editor->cursor_row());
    m_editor->insert_char(pos_to_index_ffi(*m_editor, c),
                          event->text().toStdString().c_str());
    break;
  }
  }

  repaint();
}

CursorFFI EditorWidget::makeCursorFFI(std::size_t row, std::size_t column) {
  const auto c = CursorFFI{
      .row = m_editor->cursor_row(),
      .column = m_editor->cursor_col(),
      .sticky_column = m_editor->cursor_col(),
  };

  return c;
}
