//
//  EmojiArtApp.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/7/30.
//

import SwiftUI

@main
struct EmojiArtApp: App {
    let document = EmojiArtDocument()
    var body: some Scene {
        WindowGroup {
            EmojiArtDocumentView(document: document)
        }
    }
}
