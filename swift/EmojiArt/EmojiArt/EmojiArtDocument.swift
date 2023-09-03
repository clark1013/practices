//
//  EmojiArtDocument.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/7/30.
//

import SwiftUI

class EmojiArtDocument: ObservableObject {
    @Published private(set) var emojiArt: EmojiArtModel {
        didSet {
            scheduleAutoSave()
            if emojiArt.background != oldValue.background {
                fetchBackgroundImageIfNessasary()
            }
        }
    }
    
    private var autoSaveTimer: Timer?
    
    private func scheduleAutoSave() {
        autoSaveTimer?.invalidate()
        autoSaveTimer = Timer.scheduledTimer(withTimeInterval: AutoSave.coalecscingInterval, repeats: false) { _ in
            self.autosave()
        }
    }
    
    init() {
        if let url = AutoSave.url, let emojiArtModel = try? EmojiArtModel(url: url) {
            emojiArt = emojiArtModel
            fetchBackgroundImageIfNessasary()
        } else {
            emojiArt = EmojiArtModel()
        }
    }
    
    private struct AutoSave {
        static let filename = "Autosave.emojiarat"
        static var url: URL? {
            let documentDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first
            return documentDir?.appendingPathComponent(filename)
        }
        static let coalecscingInterval = 5.0
    }
    
    private func autosave() {
        if let url = AutoSave.url {
            save(to: url)
        }
    }
    
    private func save(to url: URL) {
        let thisFunc = "\(String(describing: self)).\(#function)"
        do {
            let data: Data = try emojiArt.json()
            try data.write(to: url)
            print("\(thisFunc) success!, data=\(String(data: data, encoding: .utf8) ?? "nil")")
        } catch let encodingError where encodingError is EncodingError {
            print("\(thisFunc) encode JSON failed, error=\(encodingError.localizedDescription)")
        } catch {
            print("\(thisFunc) error=\(error)")
        }

    }
    
    var emojis: [EmojiArtModel.Emoji] { emojiArt.emojis }
    var background: EmojiArtModel.Background { emojiArt.background }
    
    @Published private(set) var backgroundImage: UIImage?
    @Published private(set) var backgroundImageFetchStatus = BackgroudImageFetchStatus.idle
    
    enum BackgroudImageFetchStatus {
        case idle
        case fetching
    }
    
    private func fetchBackgroundImageIfNessasary() {
        backgroundImage = nil
        switch emojiArt.background {
        case .url(let url):
            self.backgroundImageFetchStatus = .fetching
            DispatchQueue.global(qos: .userInitiated).async {
                let data = try? Data(contentsOf: url)
                DispatchQueue.main.async { [weak self] in
                    self?.backgroundImageFetchStatus = .idle
                    if self?.emojiArt.background == EmojiArtModel.Background.url(url) {
                        if data != nil {
                            self?.backgroundImage = UIImage(data: data!)
                        }
                    }
                }
            }
        case .imageData(let data):
            backgroundImage = UIImage(data: data)
        default:
            break
        }
    }
    
    // MARK: - Intent
    func setBackground(_ background: EmojiArtModel.Background) {
        emojiArt.background = background
    }
    
    func addEmoji(_ emojiText: String, location: (x: Int, y: Int), size: CGFloat) {
        emojiArt.addEmoji(emojiText, at: location, size: Int(size))
    }
    
    func moveEmoji(_ emoji: EmojiArtModel.Emoji, by offset: CGSize) {
        if let index = emojiArt.emojis.index(matching: emoji) {
            emojiArt.emojis[index].x += Int(offset.width)
            emojiArt.emojis[index].y += Int(offset.height)
        }
    }
    
    func scaleEmoji(_ emoji: EmojiArtModel.Emoji, by scale: CGFloat){
        if let index = emojiArt.emojis.index(matching: emoji) {
            emojiArt.emojis[index].size = Int((CGFloat(emojiArt.emojis[index].size) *
                                               scale).rounded(.toNearestOrAwayFromZero))
        }
    }
}
