import yfinance as yf
import pandas as pd
from pandas_datareader import data as pdr
import os

yf.pdr_override()

# https://pypi.org/project/yfinance/

TICKERS = [
    "SPY",
    "AAPL",
    "MSFT",
    "AMZN",
    "PYPL",
    "SQ",
    "COST",
    "WMT",
    "KO",
    "O",
    "WBA",
    "V",
    "ABNB",
    "GOOG",
    "SBUX",
]


def convert_date(d):
    arr = str(d).split("-")
    return arr[1] + "/" + arr[2].split(" ")[0] + "/" + arr[0] + " 16:00:00"


def main():
    for t in TICKERS:
        d = yf.Ticker(t)
        info = d.info
        curdf = pd.DataFrame(
            [
                [
                    info["longName"],
                    info["quoteType"],
                    info["marketCap"],
                    info["fiftyTwoWeekHigh"],
                    info["fiftyTwoWeekLow"],
                    info["forwardPE"],
                    info["forwardEps"],
                    info["longBusinessSummary"],
                ]
            ],
            columns=[
                "name",
                "type",
                "marketcap",
                "high52",
                "low52",
                "pe",
                "eps",
                "summary",
            ],
        )
        filepath = os.path.join(
            os.getcwd(), "src/assets/contemporary/" + t.lower() + ".csv"
        )
        curdf.to_csv(filepath, index=False, index_label=False)

        df = pdr.get_data_yahoo(t)
        df.index = df.index.map(convert_date)
        df = df.drop("Adj Close", 1)
        filepath = os.path.join(
            os.getcwd(), "src/assets/historical/" + t.lower() + ".csv"
        )
        df.to_csv(filepath)


main()
