# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'youtube_browser.ui'
##
## Created by: Qt User Interface Compiler version 6.10.2
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QAction, QBrush, QColor, QConicalGradient,
    QCursor, QFont, QFontDatabase, QGradient,
    QIcon, QImage, QKeySequence, QLinearGradient,
    QPainter, QPalette, QPixmap, QRadialGradient,
    QTransform)
from PySide6.QtWebEngineWidgets import QWebEngineView
from PySide6.QtWidgets import (QApplication, QDialog, QLineEdit, QSizePolicy,
    QToolBar, QVBoxLayout, QWidget)

class Ui_YoutubeBrowserDialog(object):
    def setupUi(self, YoutubeBrowserDialog):
        if not YoutubeBrowserDialog.objectName():
            YoutubeBrowserDialog.setObjectName(u"YoutubeBrowserDialog")
        YoutubeBrowserDialog.resize(1024, 768)
        self.verticalLayoutWidget = QWidget(YoutubeBrowserDialog)
        self.verticalLayoutWidget.setObjectName(u"verticalLayoutWidget")
        self.verticalLayoutWidget.setGeometry(QRect(10, 10, 1001, 741))
        self.verticalLayout = QVBoxLayout(self.verticalLayoutWidget)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.toolBar = QToolBar(self.verticalLayoutWidget)
        self.toolBar.setObjectName(u"toolBar")
        self.toolBar.setMovable(False)
        self.horizontalSpacer = QWidget()
        self.horizontalSpacer.setObjectName(u"horizontalSpacer")
        sizePolicy = QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)
        sizePolicy.setHorizontalStretch(1)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.horizontalSpacer.sizePolicy().hasHeightForWidth())
        self.horizontalSpacer.setSizePolicy(sizePolicy)
        self.toolBar.addWidget(self.horizontalSpacer)
        self.action_back = QAction(YoutubeBrowserDialog)
        self.action_back.setObjectName(u"action_back")

        self.toolBar.addAction(self.action_back)

        self.action_forward = QAction(YoutubeBrowserDialog)
        self.action_forward.setObjectName(u"action_forward")

        self.toolBar.addAction(self.action_forward)

        self.action_refresh = QAction(YoutubeBrowserDialog)
        self.action_refresh.setObjectName(u"action_refresh")

        self.toolBar.addAction(self.action_refresh)

        self.action_stop = QAction(YoutubeBrowserDialog)
        self.action_stop.setObjectName(u"action_stop")

        self.toolBar.addAction(self.action_stop)

        self.action_home = QAction(YoutubeBrowserDialog)
        self.action_home.setObjectName(u"action_home")

        self.toolBar.addAction(self.action_home)

        self.action_copy_url = QAction(YoutubeBrowserDialog)
        self.action_copy_url.setObjectName(u"action_copy_url")

        self.toolBar.addAction(self.action_copy_url)

        self.action_add_to_download = QAction(YoutubeBrowserDialog)
        self.action_add_to_download.setObjectName(u"action_add_to_download")

        self.toolBar.addAction(self.action_add_to_download)

        self.verticalLayout.addWidget(self.toolBar)

        self.le_url = QLineEdit(self.verticalLayoutWidget)
        self.le_url.setObjectName(u"le_url")
        self.le_url.setText(u"https://www.youtube.com")

        self.verticalLayout.addWidget(self.le_url)

        self.webView = QWebEngineView(self.verticalLayoutWidget)
        self.webView.setObjectName(u"webView")
        self.webView.setUrl(QUrl(u"https://www.youtube.com"))

        self.verticalLayout.addWidget(self.webView)

        self.retranslateUi(YoutubeBrowserDialog)

        QMetaObject.connectSlotsByName(YoutubeBrowserDialog)
    # setupUi

    def retranslateUi(self, YoutubeBrowserDialog):
        YoutubeBrowserDialog.setWindowTitle(QCoreApplication.translate("YoutubeBrowserDialog", u"YouTube Browser", None))
        self.toolBar.setWindowTitle(QCoreApplication.translate("YoutubeBrowserDialog", u"toolBar", None))
        self.action_back.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Back", None))
        self.action_forward.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Forward", None))
        self.action_refresh.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Refresh", None))
        self.action_stop.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Stop", None))
        self.action_home.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Home", None))
        self.action_copy_url.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Copy URL", None))
        self.action_add_to_download.setText(QCoreApplication.translate("YoutubeBrowserDialog", u"Add to Download", None))
    # retranslateUi