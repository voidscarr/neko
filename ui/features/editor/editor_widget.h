#pragma once

#include <QWidget>
#include <ffi.rs.h>

class EditorWidget : public QWidget {
  Q_OBJECT

public:
  EditorWidget(QWidget *parent = nullptr);
  ~EditorWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;
  void keyPressEvent(QKeyEvent *event) override;
  void deleteSelectionIfActive(rust::Box<Editor> &editor);
  CursorFFI makeCursorFFI(std::size_t row, std::size_t column);

  QFont m_font;
  QFontMetricsF m_fontMetrics;
  rust::Box<Editor> m_editor;

  const int CURSOR_WIDTH = 1.0;
};
