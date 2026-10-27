#pragma once

#include <QAbstractScrollArea>
#include <QScrollBar>
#include <ffi.rs.h>

class EditorWidget : public QAbstractScrollArea {
  Q_OBJECT

public:
  EditorWidget(QWidget *parent = nullptr);
  ~EditorWidget() = default;

private:
  void paintEvent(QPaintEvent *event) override;
  void keyPressEvent(QKeyEvent *event) override;
  void resizeEvent(QResizeEvent *event) override;
  void wheelEvent(QWheelEvent *event) override;
  void deleteSelectionIfActive(rust::Box<Editor> &editor);
  CursorFFI makeCursorFFI();

  QSize calculateResize();
  void performResize();
  void scrollToCursor();
  void increaseFontSize();
  void decreaseFontSize();

  QFont m_font;
  QFontMetricsF m_fontMetrics;
  rust::Box<Editor> m_editor;
  rust::Box<Config> m_config;

  const int CURSOR_WIDTH = 1.0;
  double VIEWPORT_PADDING = 100.0;
  const int LINE_DRAWING_BUFFER = 10.0;
  const int FONT_SIZE_INCREMENT = 2.0;
};
