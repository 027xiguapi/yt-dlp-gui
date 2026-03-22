try:
    from PySide6.QtWebEngineWidgets import QWebEngineView
    print("PySide6 WebEngine is available")
except ImportError:
    print("PySide6 WebEngine is NOT available")
