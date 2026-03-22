# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'sniffer_dialog.ui'
##
## Created by: Qt User Interface Compiler version 6.10.2
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QBrush, QColor, QConicalGradient, QCursor,
    QFont, QFontDatabase, QGradient, QIcon,
    QImage, QKeySequence, QLinearGradient, QPainter,
    QPalette, QPixmap, QRadialGradient, QTransform)
from PySide6.QtWidgets import (QApplication, QComboBox, QDialog, QGridLayout,
    QGroupBox, QHBoxLayout, QHeaderView, QLabel,
    QLineEdit, QPushButton, QSizePolicy, QSpacerItem,
    QStatusBar, QTabWidget, QTableWidget, QTableWidgetItem,
    QVBoxLayout, QWidget)

class Ui_SnifferDialog(object):
    def setupUi(self, SnifferDialog):
        if not SnifferDialog.objectName():
            SnifferDialog.setObjectName(u"SnifferDialog")
        SnifferDialog.resize(800, 600)
        self.verticalLayout = QVBoxLayout(SnifferDialog)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.gb_control = QGroupBox(SnifferDialog)
        self.gb_control.setObjectName(u"gb_control")
        self.gridLayout = QGridLayout(self.gb_control)
        self.gridLayout.setObjectName(u"gridLayout")
        self.lb_url = QLabel(self.gb_control)
        self.lb_url.setObjectName(u"lb_url")

        self.gridLayout.addWidget(self.lb_url, 0, 0, 1, 1)

        self.le_url = QLineEdit(self.gb_control)
        self.le_url.setObjectName(u"le_url")

        self.gridLayout.addWidget(self.le_url, 0, 1, 1, 1)

        self.cb_browser = QComboBox(self.gb_control)
        self.cb_browser.addItem("")
        self.cb_browser.addItem("")
        self.cb_browser.addItem("")
        self.cb_browser.setObjectName(u"cb_browser")

        self.gridLayout.addWidget(self.cb_browser, 0, 2, 1, 1)

        self.pb_start = QPushButton(self.gb_control)
        self.pb_start.setObjectName(u"pb_start")

        self.gridLayout.addWidget(self.pb_start, 0, 3, 1, 1)

        self.pb_stop = QPushButton(self.gb_control)
        self.pb_stop.setObjectName(u"pb_stop")
        self.pb_stop.setEnabled(False)

        self.gridLayout.addWidget(self.pb_stop, 0, 4, 1, 1)


        self.verticalLayout.addWidget(self.gb_control)

        self.tabWidget = QTabWidget(SnifferDialog)
        self.tabWidget.setObjectName(u"tabWidget")
        self.tab_videos = QWidget()
        self.tab_videos.setObjectName(u"tab_videos")
        self.verticalLayout_2 = QVBoxLayout(self.tab_videos)
        self.verticalLayout_2.setObjectName(u"verticalLayout_2")
        self.table_videos = QTableWidget(self.tab_videos)
        if (self.table_videos.columnCount() < 5):
            self.table_videos.setColumnCount(5)
        __qtablewidgetitem = QTableWidgetItem()
        self.table_videos.setHorizontalHeaderItem(0, __qtablewidgetitem)
        __qtablewidgetitem1 = QTableWidgetItem()
        self.table_videos.setHorizontalHeaderItem(1, __qtablewidgetitem1)
        __qtablewidgetitem2 = QTableWidgetItem()
        self.table_videos.setHorizontalHeaderItem(2, __qtablewidgetitem2)
        __qtablewidgetitem3 = QTableWidgetItem()
        self.table_videos.setHorizontalHeaderItem(3, __qtablewidgetitem3)
        __qtablewidgetitem4 = QTableWidgetItem()
        self.table_videos.setHorizontalHeaderItem(4, __qtablewidgetitem4)
        self.table_videos.setObjectName(u"table_videos")

        self.verticalLayout_2.addWidget(self.table_videos)

        self.horizontalLayout = QHBoxLayout()
        self.horizontalLayout.setObjectName(u"horizontalLayout")
        self.pb_add_video = QPushButton(self.tab_videos)
        self.pb_add_video.setObjectName(u"pb_add_video")

        self.horizontalLayout.addWidget(self.pb_add_video)

        self.pb_download_video = QPushButton(self.tab_videos)
        self.pb_download_video.setObjectName(u"pb_download_video")

        self.horizontalLayout.addWidget(self.pb_download_video)

        self.horizontalSpacer = QSpacerItem(40, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout.addItem(self.horizontalSpacer)


        self.verticalLayout_2.addLayout(self.horizontalLayout)

        self.tabWidget.addTab(self.tab_videos, "")
        self.tab_images = QWidget()
        self.tab_images.setObjectName(u"tab_images")
        self.verticalLayout_3 = QVBoxLayout(self.tab_images)
        self.verticalLayout_3.setObjectName(u"verticalLayout_3")
        self.table_images = QTableWidget(self.tab_images)
        if (self.table_images.columnCount() < 6):
            self.table_images.setColumnCount(6)
        __qtablewidgetitem5 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(0, __qtablewidgetitem5)
        __qtablewidgetitem6 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(1, __qtablewidgetitem6)
        __qtablewidgetitem7 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(2, __qtablewidgetitem7)
        __qtablewidgetitem8 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(3, __qtablewidgetitem8)
        __qtablewidgetitem9 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(4, __qtablewidgetitem9)
        __qtablewidgetitem10 = QTableWidgetItem()
        self.table_images.setHorizontalHeaderItem(5, __qtablewidgetitem10)
        self.table_images.setObjectName(u"table_images")

        self.verticalLayout_3.addWidget(self.table_images)

        self.horizontalLayout_2 = QHBoxLayout()
        self.horizontalLayout_2.setObjectName(u"horizontalLayout_2")
        self.pb_add_image = QPushButton(self.tab_images)
        self.pb_add_image.setObjectName(u"pb_add_image")

        self.horizontalLayout_2.addWidget(self.pb_add_image)

        self.pb_download_image = QPushButton(self.tab_images)
        self.pb_download_image.setObjectName(u"pb_download_image")

        self.horizontalLayout_2.addWidget(self.pb_download_image)

        self.horizontalSpacer_2 = QSpacerItem(40, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_2.addItem(self.horizontalSpacer_2)


        self.verticalLayout_3.addLayout(self.horizontalLayout_2)

        self.tabWidget.addTab(self.tab_images, "")
        self.tab_statistics = QWidget()
        self.tab_statistics.setObjectName(u"tab_statistics")
        self.verticalLayout_4 = QVBoxLayout(self.tab_statistics)
        self.verticalLayout_4.setObjectName(u"verticalLayout_4")
        self.lb_stats = QLabel(self.tab_statistics)
        self.lb_stats.setObjectName(u"lb_stats")

        self.verticalLayout_4.addWidget(self.lb_stats)

        self.pb_export = QPushButton(self.tab_statistics)
        self.pb_export.setObjectName(u"pb_export")

        self.verticalLayout_4.addWidget(self.pb_export)

        self.tabWidget.addTab(self.tab_statistics, "")

        self.verticalLayout.addWidget(self.tabWidget)

        self.statusBar = QStatusBar(SnifferDialog)
        self.statusBar.setObjectName(u"statusBar")

        self.verticalLayout.addWidget(self.statusBar)


        self.retranslateUi(SnifferDialog)

        self.tabWidget.setCurrentIndex(0)


        QMetaObject.connectSlotsByName(SnifferDialog)
    # setupUi

    def retranslateUi(self, SnifferDialog):
        SnifferDialog.setWindowTitle(QCoreApplication.translate("SnifferDialog", u"Browser Sniffer - YouTube Resource Capture", None))
        self.gb_control.setTitle(QCoreApplication.translate("SnifferDialog", u"Control", None))
        self.lb_url.setText(QCoreApplication.translate("SnifferDialog", u"YouTube URL:", None))
        self.le_url.setText(QCoreApplication.translate("SnifferDialog", u"https://www.youtube.com", None))
        self.cb_browser.setItemText(0, QCoreApplication.translate("SnifferDialog", u"Chrome", None))
        self.cb_browser.setItemText(1, QCoreApplication.translate("SnifferDialog", u"Firefox", None))
        self.cb_browser.setItemText(2, QCoreApplication.translate("SnifferDialog", u"Edge", None))

        self.pb_start.setText(QCoreApplication.translate("SnifferDialog", u"Start Capture", None))
        self.pb_stop.setText(QCoreApplication.translate("SnifferDialog", u"Stop", None))
        ___qtablewidgetitem = self.table_videos.horizontalHeaderItem(0)
        ___qtablewidgetitem.setText(QCoreApplication.translate("SnifferDialog", u"URL", None));
        ___qtablewidgetitem1 = self.table_videos.horizontalHeaderItem(1)
        ___qtablewidgetitem1.setText(QCoreApplication.translate("SnifferDialog", u"Type", None));
        ___qtablewidgetitem2 = self.table_videos.horizontalHeaderItem(2)
        ___qtablewidgetitem2.setText(QCoreApplication.translate("SnifferDialog", u"Size", None));
        ___qtablewidgetitem3 = self.table_videos.horizontalHeaderItem(3)
        ___qtablewidgetitem3.setText(QCoreApplication.translate("SnifferDialog", u"MIME Type", None));
        ___qtablewidgetitem4 = self.table_videos.horizontalHeaderItem(4)
        ___qtablewidgetitem4.setText(QCoreApplication.translate("SnifferDialog", u"Action", None));
        self.pb_add_video.setText(QCoreApplication.translate("SnifferDialog", u"Add Selected to Download List", None))
        self.pb_download_video.setText(QCoreApplication.translate("SnifferDialog", u"Download Selected", None))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_videos), QCoreApplication.translate("SnifferDialog", u"Videos", None))
        ___qtablewidgetitem5 = self.table_images.horizontalHeaderItem(0)
        ___qtablewidgetitem5.setText(QCoreApplication.translate("SnifferDialog", u"URL", None));
        ___qtablewidgetitem6 = self.table_images.horizontalHeaderItem(1)
        ___qtablewidgetitem6.setText(QCoreApplication.translate("SnifferDialog", u"Type", None));
        ___qtablewidgetitem7 = self.table_images.horizontalHeaderItem(2)
        ___qtablewidgetitem7.setText(QCoreApplication.translate("SnifferDialog", u"Size", None));
        ___qtablewidgetitem8 = self.table_images.horizontalHeaderItem(3)
        ___qtablewidgetitem8.setText(QCoreApplication.translate("SnifferDialog", u"MIME Type", None));
        ___qtablewidgetitem9 = self.table_images.horizontalHeaderItem(4)
        ___qtablewidgetitem9.setText(QCoreApplication.translate("SnifferDialog", u"Preview", None));
        ___qtablewidgetitem10 = self.table_images.horizontalHeaderItem(5)
        ___qtablewidgetitem10.setText(QCoreApplication.translate("SnifferDialog", u"Action", None));
        self.pb_add_image.setText(QCoreApplication.translate("SnifferDialog", u"Add Selected to Download List", None))
        self.pb_download_image.setText(QCoreApplication.translate("SnifferDialog", u"Download Selected", None))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_images), QCoreApplication.translate("SnifferDialog", u"Images", None))
        self.lb_stats.setText(QCoreApplication.translate("SnifferDialog", u"Statistics will appear here...", None))
        self.pb_export.setText(QCoreApplication.translate("SnifferDialog", u"Export Captured Data", None))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_statistics), QCoreApplication.translate("SnifferDialog", u"Statistics", None))
    # retranslateUi

