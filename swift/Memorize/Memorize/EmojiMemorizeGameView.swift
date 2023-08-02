//
//  ContentView.swift
//  Memorize
//
//  Created by Wen Jiazhi on 2023/7/5.
//

import SwiftUI

struct EmojiMemorizeGameView: View {
    @ObservedObject var viewModel = EmojiMemorizeGame();
    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 75))]) {
                ForEach(viewModel.cards) { card in
                    CardView(card: card)
                        .aspectRatio(2/3, contentMode: .fit)
                        .onTapGesture {
                            viewModel.choose(card);
                        }
                }
            }
        }
        .padding(.horizontal)
        .foregroundColor(.red)
    }
    

}

struct CardView: View {
    let card: MemorizeGame<String>.Card
    var body: some View {
        let shape = RoundedRectangle(cornerRadius: 16)
        ZStack {
            if card.isFaceUp {
                shape.fill().foregroundColor(.white)
                shape.strokeBorder(lineWidth: 3)
                Text(card.content)
            } else if card.isMatched {
                shape.opacity(0)
            } else {
                shape.fill()
            }
        }
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        EmojiMemorizeGameView().previewDisplayName("Light")
        EmojiMemorizeGameView()
            .preferredColorScheme(.dark)
            .previewDisplayName("Dark")
    }
}
