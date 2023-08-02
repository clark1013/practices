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
            if emojiArt.background != oldValue.background {
                fetchBackgroundImageIfNessasary()
            }
        }
    }
    
    init() {
        emojiArt = EmojiArtModel()
        emojiArt.addEmoji("😁", at: (x: 0, y: 0), size: 80)
        emojiArt.addEmoji("🙂", at: (x: 100, y: 100), size: 30)
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
