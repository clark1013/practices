//
//  EmojiMemorizeGame.swift
//  Memorize
//
//  Created by Wen Jiazhi on 2023/7/12.
//

import Foundation

class EmojiMemorizeGame: ObservableObject {
    static let emojis = ["🚗", "🛴", "✈️", "🛵", "⛵️", "🚎", "🚐", "🚛", "🛻", "🏎", "🚂", "🚊", "🚀", "🚁", "🚢", "🛶", "🛥", "🚞", "🚟", "🚃", "🍆", "🫛", "🍕", "🥚"]
    
    @Published private var model: MemorizeGame<String> = MemorizeGame(numbersOfCardPairs: 4) {index in
        emojis[index]
    }

    var cards: Array<MemorizeGame<String>.Card> {model.cards}
    
    func choose(_ card: MemorizeGame<String>.Card) {
        model.choose(card);
    }
}
