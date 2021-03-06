import matplotlib.pyplot as plt

def test():
    print("This is a test.  Only a test.")

def plot(initial_age, savings_history):
    print("Calling the python plotting function ...")
    age = list(range(initial_age, initial_age + len(savings_history)))
    assert len(age) == len(savings_history), "Problem with data lengths. len(age) = {} len(savings_history) == {}".format(len(age), len(savings_history))
    print("savings_history:", savings_history)
    plt.plot(age, savings_history)
    plt.show()

def plot_test():
    age = 30;
    savings_history = [1, 2, 4, 8, 16, 32, 64, 128];
    plot(age, savings_history);

if __name__ == "__main__":
    plot_test()
