import { Button, twMerge, useWizard } from "@dango/shared";
import type React from "react";
import type { PropsWithChildren } from "react";
import { useNavigate } from "react-router-dom";

export const WizardSignupWrapper: React.FC<PropsWithChildren> = ({ children }) => {
  const { activeStep, previousStep } = useWizard();
  const navigate = useNavigate();

  const isFirstStep = activeStep === 0;
  const isSecondStep = activeStep === 1;
  const isThirdStep = activeStep === 2;

  return (
    <div className="flex flex-col items-center justify-center w-full gap-8">
      <div className="flex flex-col items-center w-full bg-surface-rose-100 px-8 py-6 rounded-3xl max-w-2xl gap-12 shadow-md">
        <div className="flex flex-col gap-8 md:gap-10 w-full">
          <div className="flex flex-col gap-4 items-center">
            <p className="font-extrabold text-typography-rose-700 tracking-widest uppercase text-lg">
              {[0, 1].includes(activeStep) ? "signup" : null}
              {isThirdStep ? "new spot account" : null}
            </p>
            <p className="text-typography-rose-600 text-lg text-center">
              {isFirstStep
                ? "Choose your username. It will be public onchain and cannot be changed afterwards."
                : null}
              {isSecondStep
                ? "Choose a sign-in credential. You can add or remove credentials afterwards."
                : null}
              {isThirdStep
                ? "Fund your first spot account with USDC from other existing wallets of yours."
                : null}
            </p>
          </div>
          <div className="flex flex-1 justify-center items-center w-full">{children}</div>
        </div>
        <div className="flex gap-4">
          <p
            className={twMerge(
              "text-[10px] font-semibold tracking-[0.125rem]",
              isFirstStep ? "text-typography-purple-400" : "text-typography-purple-300",
            )}
          >
            1 USERNAME
          </p>
          <p
            className={twMerge(
              "text-[10px] font-semibold tracking-[0.125rem]",
              isSecondStep ? "text-typography-purple-400" : "text-typography-purple-300",
            )}
          >
            2 CREDENTIAL
          </p>
          <p
            className={twMerge(
              "text-[10px] font-semibold tracking-[0.125rem]",
              isThirdStep ? "text-typography-purple-400" : "text-typography-purple-300",
            )}
          >
            3 DEPOSIT
          </p>
        </div>
      </div>
      <Button
        type="button"
        variant="light"
        color="rose"
        className="italic"
        onClick={() => (isFirstStep ? navigate("/auth/login") : previousStep())}
      >
        {isFirstStep ? "Already have an account?" : "Back"}
      </Button>
    </div>
  );
};
