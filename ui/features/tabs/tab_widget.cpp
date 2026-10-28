#include "tab_widget.h"
#include <QLabel>
#include <QPainter>
#include <QPushButton>

TabWidget::TabWidget(bool active, bool dirty, std::string title,
                     std::string path, rust::Box<Config> &config,
                     rust::Box<ThemeManager> &themeManager, QWidget *parent)
    : m_config(config), m_themeManager(themeManager) {
  setAutoFillBackground(false);
  setFocusPolicy(Qt::FocusPolicy::NoFocus);
  setMaximumWidth(100);

  m_hBoxLayout = new QHBoxLayout(this);
  m_hBoxLayout->setContentsMargins(0, 0, 0, 0);
  m_hBoxLayout->setSpacing(10);
  m_hBoxLayout->setAlignment(Qt::AlignVCenter);

  QLabel *label = new QLabel(title.c_str(), this);
  QPushButton *button = new QPushButton("x", this);

  button->setMaximumWidth(20);
  button->setStyleSheet("padding: 4px; border: none;");

  m_hBoxLayout->addWidget(label);
  m_hBoxLayout->addWidget(button);
}
