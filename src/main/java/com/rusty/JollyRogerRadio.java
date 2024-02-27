package com.rusty;

public class JollyRogerRadio {

    static {
        System.loadLibrary("rusty");
    }

    public static void main(String args[]) {
        long startTime = System.currentTimeMillis();
        int numberOfTrial = 100000;
        int numberOfIteration = 100000;
        int error = calculateError(numberOfTrial, numberOfIteration);
        long endTime = System.currentTimeMillis();
        long nbJniTransition = numberOfIteration;
        System.out.println("Java -> Rust -> Rust : res = "+error+" Total execution time for JNI transitions (J->R) = "+nbJniTransition+" \t\t: " + (endTime-startTime) + "ms");

    }
    public static int calculateError(int numberOfTrials, int numberOfIterations) {
        int error = 0;
        for (int i = 0; i < numberOfTrials; i++) {
            error += 50 - calc(numberOfIterations);
        }
        return error;
    }

    private native static int calc(int n);

}
