#include "main_container_widget.h"

MainContainerWidget::MainContainerWidget(QWidget *parent)
    : m_tabManager(new_tab_manager()), m_editor(new_editor()),
      m_config(new_config()), m_themeManager(new_theme_manager()) {
  setAutoFillBackground(false);
  setContentsMargins(0, 0, 0, 0);

  setFocusPolicy(Qt::FocusPolicy::NoFocus);

  m_vBoxLayout = new QVBoxLayout(this);
  m_editorWidget = new EditorWidget(m_editor, m_config, m_themeManager, this);
  m_tabContainerWidget =
      new TabContainerWidget(m_tabManager, m_config, m_themeManager, this);

  m_vBoxLayout->setContentsMargins(0, 0, 0, 0);
  m_vBoxLayout->setSpacing(0);

  m_vBoxLayout->addWidget(m_tabContainerWidget);
  m_vBoxLayout->addWidget(m_editorWidget);
}
