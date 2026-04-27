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

  QFont m_font;
  QFontMetricsF m_fontMetrics;
  rust::Box<Editor> m_editor;
};
