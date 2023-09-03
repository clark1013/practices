//
//  EmojiArtApp.swift
//  EmojiArt
//
//  Created by Wen Jiazhi on 2023/7/30.
//

import SwiftUI

@main
struct EmojiArtApp: App {
    @StateObject var document = EmojiArtDocument()
    @StateObject var paletteStore = PaletteStore(name: "Default")

    var body: some Scene {
        WindowGroup {
            EmojiArtDocumentView(document: document)
                .environmentObject(paletteStore)
        }
    }
}
