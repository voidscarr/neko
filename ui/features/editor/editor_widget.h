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

  QFont m_font;
  QFontMetricsF m_fontMetrics;
  Buffer *m_buffer;
};
