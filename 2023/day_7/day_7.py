import enum


class Hand:
    def __init__(self, name, value) -> None:
        self.name = name
        self.value = value
        self.score: int

    def set_score(self, score: int) -> None:
        self.score = score

    def set_rank(self) -> None:
        """set rank depenings on amount of pairs"""
        types = [self.name.count(char) for char in self.name]
        rank: list = []
        for times in types:  # times = 2 | ranks = [2,2,1,2,2]
            c = types.count(times)  # c = 4
            if times == 1 and 1 not in rank:
                rank.append(1)
            if times != 1 and rank.count(times) != c / times:
                rank.append(times)
        self.set_score(self.set_score_card(rank))

    def set_score_card(self, rank) -> int:
        """gives the score depending on hand"""
        score = 0
        if rank.count(5) == 1:
            score = 7
        elif rank.count(4) == 1:
            score = 6
        elif rank.count(3) == 1 and rank.count(2):
            score = 5
        elif rank.count(3) == 1:
            score = 4
        elif rank.count(2) == 2:
            score = 3
        elif rank.count(2) == 1:
            score = 2
        else:
            score = 1
        return score

    def __str__(self) -> str:
        return f"{self.name}, {self.value}, {self.score}"


def get_cards(txt: str) -> list:
    """read from file"""
    with open(txt, "r") as file:
        cards = []
        for line in file.readlines():
            card = line.strip().split(" ")
            hand = Hand(name=card[0], value=int(card[1]))
            cards.append(hand)
        return cards


def get_face(face: str) -> int:
    if face == "A":
        return 14
    elif face == "K":
        return 13
    elif face == "Q":
        return 12
    elif face == "J":
        return 11
    elif face == "T":
        return 10
    else:
        raise ValueError


def get_winnings(hand: list) -> int:
    """multiple the bid with winning order"""
    points = 0
    for i, card in enumerate(hand, 1):
        points += i * card.value
    return points


def quick_sort(cards: list) -> list:
    if len(cards) < 2:
        return cards

    else:
        pivot = cards[0]
        less = []
        greater = []
        # print(len(cards))

        # i = 1
        for card in cards[1:]:
            if card.score < pivot.score:
                less.append(card)
                # print(
                #     f"({pivot.name}, {pivot.score}) adding {card.name}-{card.score} to less"
                # )
            elif card.score == pivot.score:
                # print("in card == pivot")
                for index in range(len(card.name)):
                    card_value = (
                        int(card.name[index])
                        if card.name[index].isdigit()
                        else get_face(card.name[index])
                    )
                    pivot_value = (
                        int(pivot.name[index])
                        if pivot.name[index].isdigit()
                        else get_face(pivot.name[index])
                    )
                    # print(card_value)
                    # print(f"card ({card.name}) : {card.name[index]} - {card_value}")
                    # print(f"card ({pivot.name}) : {pivot.name[index]} - {pivot_value}")
                    if card_value < pivot_value:
                        less.append(card)
                        # print(
                        #     f"({pivot.name}, {pivot.score}) adding {card.name}-{card.score} to less"
                        # )
                        # print(f"adding {card.name} to less")
                        break
                    if card_value > pivot_value:
                        break

        for card in cards[1:]:
            if card.score > pivot.score:
                greater.append(card)
                # print(
                #     f"({pivot.name}, {pivot.score}) adding {card.name}-{card.score} to greater"
                # )
            elif card.score == pivot.score:
                # print("in card == pivot")
                # print("............................")
                # print(f"card in loop : {card.name}")
                for index in range(len(card.name)):
                    card_value = (
                        int(card.name[index])
                        if card.name[index].isdigit()
                        else get_face(card.name[index])
                    )
                    pivot_value = (
                        int(pivot.name[index])
                        if pivot.name[index].isdigit()
                        else get_face(pivot.name[index])
                    )
                    """ something here is wrong!!!"""
                    # print(f"card ({card.name}) : {card.name[index]} - {card_value}")
                    # print(f"card ({pivot.name}) : {pivot.name[index]} - {pivot_value}")
                    # print(card_value)
                    # print(card_value, pivot_value)
                    if card_value > pivot_value:
                        # print(card_value, pivot_value)
                        greater.append(card)
                        # print(
                        #     f"({pivot.name}, {pivot.score}) adding {card.name}-{card.score} to greater"
                        # )
                        break
                    elif card_value < pivot_value:
                        break

        # print("end of line")
        # print("************")
        return quick_sort(less) + [pivot] + quick_sort(greater)


def get_score(cards: dict):
    index = 1
    for card in cards:
        print(index, card, cards[card])
        index += 1
    print(list(cards)[0])
    return cards


def camel_poker():
    hand = get_cards("day_7_input.txt")
    for cards in hand:
        cards.set_rank()
        # print(cards)

    # print("************")
    hand = quick_sort(hand)
    # print("-------")
    # for i in hand:
    #     print(i)
    win = get_winnings(hand)
    print(win)


if __name__ == "__main__":
    camel_poker()
