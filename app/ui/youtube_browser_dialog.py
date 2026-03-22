from PySide6 import QtCore, QtGui, QtWidgets
from ui.youtube_browser import Ui_YoutubeBrowserDialog
from utils import ROOT

class YoutubeBrowserDialog(QtWidgets.QDialog, Ui_YoutubeBrowserDialog):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.setWindowIcon(QtGui.QIcon(str(ROOT / "assets" / "yt-dlp-gui.ico")))
        
        # 设置图标
        import qtawesome as qta
        self.action_back.setIcon(qta.icon("mdi6.arrow-left"))
        self.action_forward.setIcon(qta.icon("mdi6.arrow-right"))
        self.action_refresh.setIcon(qta.icon("mdi6.refresh"))
        self.action_stop.setIcon(qta.icon("mdi6.stop"))
        self.action_home.setIcon(qta.icon("mdi6.home"))
        self.action_copy_url.setIcon(qta.icon("mdi6.content-copy"))
        self.action_add_to_download.setIcon(qta.icon("mdi6.download"))
        
        # 连接信号
        self.connect_signals()
        
        # 初始化 WebEngine 配置
        self.init_web_engine()
    
    def connect_signals(self):
        """连接信号槽"""
        # 工具栏按钮
        self.action_back.triggered.connect(self.webView.back)
        self.action_forward.triggered.connect(self.webView.forward)
        self.action_refresh.triggered.connect(self.webView.reload)
        self.action_stop.triggered.connect(self.webView.stop)
        self.action_home.triggered.connect(self.go_home)
        self.action_copy_url.triggered.connect(self.copy_url)
        self.action_add_to_download.triggered.connect(self.add_to_download)
        
        # URL 输入框
        self.le_url.returnPressed.connect(self.load_url)
        
        # WebView 信号
        self.webView.urlChanged.connect(self.on_url_changed)
        self.webView.loadStarted.connect(self.on_load_started)
        self.webView.loadFinished.connect(self.on_load_finished)
    
    def init_web_engine(self):
        """初始化 WebEngine 配置"""
        # 启用 JavaScript
        settings = self.webView.settings()
        settings.setAttribute(settings.WebAttribute.JavascriptEnabled, True)
        settings.setAttribute(settings.WebAttribute.PluginsEnabled, True)
        settings.setAttribute(settings.WebAttribute.AutoLoadImages, True)
        settings.setAttribute(settings.WebAttribute.LocalStorageEnabled, True)
        
        # 用户代理设置在 PySide6 中通过其他方式设置
        # 暂时注释掉，使用默认用户代理
    
    def go_home(self):
        """回到首页"""
        self.webView.setUrl(QtCore.QUrl("https://www.youtube.com"))
    
    def load_url(self):
        """加载 URL"""
        url = self.le_url.text().strip()
        if url:
            if not url.startswith("http"):
                url = "https://" + url
            self.webView.setUrl(QtCore.QUrl(url))
    
    def on_url_changed(self, url):
        """URL 改变时更新输入框"""
        self.le_url.setText(url.toString())
    
    def on_load_started(self):
        """开始加载时的处理"""
        self.setWindowTitle(f"YouTube Browser - Loading...")
    
    def on_load_finished(self, ok):
        """加载完成时的处理"""
        if ok:
            title = self.webView.title()
            if title:
                self.setWindowTitle(f"YouTube Browser - {title}")
            else:
                self.setWindowTitle("YouTube Browser")
        else:
            self.setWindowTitle("YouTube Browser - Error")
    
    def copy_url(self):
        """复制当前 URL"""
        url = self.webView.url().toString()
        QtWidgets.QApplication.clipboard().setText(url)
        self.statusBar().showMessage("URL copied to clipboard", 2000)
    
    def add_to_download(self):
        """添加当前视频到下载列表"""
        url = self.webView.url().toString()
        if "youtube.com" in url or "youtu.be" in url:
            # 发送信号给主窗口
            self.parent().add_url_to_download(url)
            self.statusBar().showMessage("Added to download list", 2000)
        else:
            QtWidgets.QMessageBox.warning(self, "Warning", "This is not a YouTube video URL")
    
    def closeEvent(self, event):
        """关闭事件"""
        # 清理 WebEngine 资源
        self.webView.stop()
        event.accept()
