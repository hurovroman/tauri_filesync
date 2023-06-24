import PageLayout from "@/components/layout/PageLayout";

/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"SendFile Help"} includeSearchBar={true}>
        <div>
          <h1>Help</h1>

          <div>some help content goes here</div>
        </div>
      </PageLayout>
    </>
  );
}
