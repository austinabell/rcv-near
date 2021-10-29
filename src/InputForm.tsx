import React, { FC, useState } from "react";

interface IProps {
  initializeCandidates: (candidates: string[]) => void;
}

export const InputForm: FC<IProps> = ({ initializeCandidates }: IProps) => {
  const [inputList, setInputList] = useState([""]);

  // handle input change
  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement>,
    index: number
  ) => {
    const { value } = e.target;
    const list = [...inputList];
    list[index] = value;
    setInputList(list);
  };

  // handle click event of the Remove button
  const handleRemoveClick = (index: number) => {
    const list = [...inputList];
    list.splice(index, 1);
    setInputList(list);
  };

  // handle click event of the Add button
  const handleAddClick = () => {
    setInputList([...inputList, ""]);
  };

  const handleInitialize = () => {
    initializeCandidates(inputList.filter((v) => v.trim().length != 0));
  };

  return (
    <div className="App">
      {inputList.map((x, i) => {
        return (
          <div className="container">
            <input
              name="cadndidate"
              placeholder="Enter Candidate Name"
              value={x}
              onChange={(e) => handleInputChange(e, i)}
            />
            <div className="btn-box">
              {inputList.length !== 1 && (
                <button className="mr10" onClick={() => handleRemoveClick(i)}>
                  Remove
                </button>
              )}
              {inputList.length - 1 === i && (
                <button onClick={handleAddClick}>Add</button>
              )}
            </div>
          </div>
        );
      })}
      <button
        onClick={handleInitialize}
        style={{ width: 400, backgroundColor: "#303030" }}
      >
        Initialize Candidates
      </button>
    </div>
  );
};
