import {
  FC,
  useState,
  useCallback,
  SyntheticEvent,
  useEffect,
  Dispatch,
  SetStateAction,
} from "react";
import { Card } from "./Card";
import update from "immutability-helper";
import React from "react";
import { InputForm } from "./InputForm";

const style = {
  width: 400,
};

export interface Item {
  id: number;
  text: string;
}

interface IProps {
  setShowNotification: Dispatch<SetStateAction<boolean>>;
}

export const Container: FC<IProps> = ({ setShowNotification }: IProps) => {
  const [cards, setCards] = useState<Array<Item>>([]);
  const [requireInit, setRequireInit] = useState<boolean>(false);

  const [winner, setWinner] = useState("");

  useEffect(() => {
    async function initializeCandidates() {
      return window.contract
        .get_candidates()
        .then((candidate_names: Array<string>) => {
          let candidates = [];
          for (let i = 0; i < candidate_names.length; i++) {
            candidates.push({ id: i, text: candidate_names[i] });
          }

          return candidates;
        });
    }
    initializeCandidates()
      .then((candidates) => setCards(candidates))
      .catch(() => {
        setRequireInit(true);
      });
  }, []);

  const moveCard = useCallback(
    (dragIndex: number, hoverIndex: number) => {
      const dragCard = cards[dragIndex];
      setCards(
        update(cards, {
          $splice: [
            [dragIndex, 1],
            [hoverIndex, 0, dragCard],
          ],
        })
      );
    },
    [cards]
  );

  const renderCard = (card: { id: number; text: string }, index: number) => {
    return (
      <Card
        key={card.id}
        index={index}
        id={card.id}
        text={card.text}
        moveCard={moveCard}
      />
    );
  };

  const getWinner = (e: SyntheticEvent) => {
    e.preventDefault();
    window.contract.get_winner().then((winner: string | null) => {
      if (winner) {
        setWinner(winner);
      }
    });
  };

  const submitVote = async (event: SyntheticEvent) => {
    event.preventDefault();

    try {
      // make an update call to the smart contract
      await window.contract.vote({ order: cards.map((v) => v.text) });
    } catch (e) {
      alert(
        "Something went wrong! " +
          "Maybe you need to sign out and back in? " +
          "Check your browser console for more info."
      );
      throw e;
    }

    // show Notification
    setShowNotification(true);

    // remove Notification again after css animation completes
    // this allows it to be shown again next time the form is submitted
    setTimeout(() => {
      setShowNotification(false);
    }, 11000);
  };

  const initializeCandidates = (candidates: string[]) => {
    window.contract
      .new({
        candidates,
      })
      .then(() => setRequireInit(false));
  };

  const voteCards = () => {
    return (
      <>
        <div style={style}>{cards.map((card, i) => renderCard(card, i))}</div>
        <button onClick={submitVote} style={style}>
          Vote
        </button>
        <div style={{ marginBottom: "8px" }} />
        <button
          onClick={getWinner}
          style={{ width: 400, backgroundColor: "#303030" }}
        >
          Calculate Winner
        </button>
        <h2>{winner && "Winner is: " + winner}</h2>
      </>
    );
  };

  return (
    <div
      style={{
        position: "absolute",
        left: "50%",
        top: "35%",
        transform: "translate(-50%, -50%)",
      }}
    >
      <h1>Ranked Choice Voting</h1>
      {requireInit ? (
        <InputForm initializeCandidates={initializeCandidates} />
      ) : (
        voteCards()
      )}
    </div>
  );
};
