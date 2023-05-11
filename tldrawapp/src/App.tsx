import { useEffect, useState } from "react";
import { Button, Flex, Text } from "@chakra-ui/react";

function timeString(t: number) {
  return `${Math.floor(t / 60) < 10
    ? `0${Math.floor(t / 60)}`
    : `${Math.floor(t / 60)}`
    }:${t % 60 < 10 ? `0${t % 60}` : t % 60}`;
}

function App() {
  const [time, setTime] = useState(0);
  const [timerStart, setTimerStart] = useState(false);
  const buttons = [
    {
      value: 900,
      display: "15 minutes"
    },
    {
      value: 1800,
      display: "30 minutes"
    },
    {
      value: 3600,
      display: "1 hour"
    }
  ];

  const toggleTimer = () => {
    setTimerStart(!timerStart);
  };



  useEffect(() => {
    const interval = setInterval(() => {
      if (timerStart) {
        if (time > 0) {
          setTime(time - 1);
        } else if (time == 0) {
          clearInterval(interval);
        }
      }
    }, 1000);
    return () => clearInterval(interval);
  }, [timerStart, time]);

  return (
    <div className="App" style={{ height: "100%" }}>
      <Flex
        height="100%"
        background="gray.700"
        alignItems="center"
        flexDirection="column"
      >
        <Text fontSize="6xl" color="white" marginTop="20" fontWeight="bold">
          Promodoro Timer
        </Text>
        <Text fontWeight="bold" fontSize="7xl" color="white">
          {timeString(time)}
        </Text>
        <Flex>
          <Button
            width="7rem"
            background="tomato"
            color="white"
            onClick={() => toggleTimer()}>
            {timerStart ? "Pause" : "Start"}
          </Button>
          <Flex marginTop={10}>
            {buttons.map(({ value, display }) => (
              <Button
                marginX={4}
                background="green.300"
                color="white"
                onClick={() => {
                  setTimerStart(false);
                  setTime(value);
                }}>
                {display}
              </Button>
            ))}
          </Flex>
        </Flex>
      </Flex>
    </div >
  );
}

export default App;
