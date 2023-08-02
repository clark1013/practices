//
//  MemorizeGame.swift
//  Memorize
//
//  Created by Wen Jiazhi on 2023/7/12.
//

import Foundation

struct MemorizeGame<CardContent> where CardContent: Equatable {
    struct Card: Identifiable {
        var isFaceUp: Bool = false;
        var isMatched: Bool = false;
        var content: CardContent;
        var id: Int;
    }
    
    private(set) var cards: Array<Card>;
    
    private var theOnlyFaceUpIndex: Int? {
        get{
            cards.indices.filter({cards[$0].isFaceUp}).oneAndOnly
        }
        set{
            cards.indices.forEach({ cards[$0].isFaceUp = ($0 == newValue)})
        }
    };
    
    init(numbersOfCardPairs: Int, createCardContent: (Int) -> CardContent) {
        cards = Array<Card>();
        for i in 0..<numbersOfCardPairs {
            let content = createCardContent(i);
            cards.append(Card(content: content, id: 2*i));
            cards.append(Card(content: content, id: 2*i+1));
        }
    }
    
    mutating func choose(_ card: Card) {
        if let chosenIndex = cards.firstIndex(where: { cardsItem in cardsItem.id == card.id }),
           !cards[chosenIndex].isMatched,
           !cards[chosenIndex].isFaceUp
        {
            if let faceUpIndex = theOnlyFaceUpIndex {
                if cards[chosenIndex].content == cards[faceUpIndex].content {
                    cards[chosenIndex].isMatched = true;
                    cards[faceUpIndex].isMatched = true;
                }
                cards[chosenIndex].isFaceUp = true;
            } else {
                theOnlyFaceUpIndex = chosenIndex;
            }
        }
    }
}

extension Array {
    var oneAndOnly: Element? {
        if count == 1 {
            return first
        } else {
            return nil
        }
    }
}
