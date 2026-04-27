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

  // Text
  // TODO: Add theming
  painter.setPen("white");
  painter.setFont(m_font);

  // TODO: Implement line wrapping option toggle
  QTextOption qTextOption;
  qTextOption.setWrapMode(QTextOption::NoWrap);

  painter.drawText(rect(),
                   m_editor->content_slice(0, m_editor->content_len()).c_str(),
                   qTextOption);
}

void EditorWidget::keyPressEvent(QKeyEvent *event) {
  auto ctrlActive =
      event->modifiers().testFlag(Qt::KeyboardModifier::ControlModifier);
  // auto cmdActive =
  //     event->modifiers().testFlag(Qt::KeyboardModifier::MetaModifier);
  // auto altActive =
  //     event->modifiers().testFlag(Qt::KeyboardModifier::AltModifier);

  if (ctrlActive) {
    switch (event->key()) {
    case Qt::Key_A:
      // TODO: Add select all
      repaint();
      return;
    case Qt::Key_V: {
      const std::string clipboardText =
          QApplication::clipboard()->text().toStdString();

      m_editor->insert_char(clipboardText.c_str());

      repaint();
      return;
    }
    case Qt::Key_C:
    case Qt::Key_X: {
      // TODO: Add actual selections
      const std::string text = "test";
      QApplication::clipboard()->setText(text.c_str());

      repaint();
      return;
    }
    }
  }

  switch (event->key()) {
  case Qt::Key_Enter:
  case Qt::Key_Return:
    m_editor->insert_char("\n");
    break;
  case Qt::Key_Tab:
    // TODO: Make this configurable for \t or 2/4/8+ spaces
    m_editor->insert_char("    ");
    break;
  case Qt::Key_Backspace:
    m_editor->remove_char(m_editor->content_len() - 1, m_editor->content_len());
    break;
  default:
    m_editor->insert_char(event->text().toStdString().c_str());
    break;
  }

  repaint();
}
