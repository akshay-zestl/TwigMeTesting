import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper
import groovy.json.JsonOutput as JsonOutput
import static org.assertj.core.api.Assertions.*

def CARD_IDS = GlobalVariable.CARD_IDS

for (def index : (0..CARD_IDS.size())) {
    if (CARD_IDS[index]) {
        response = WS.sendRequest(findTestObject('Get Card', [('BASE_URL') : GlobalVariable.BASE_URL, ('API_VERSION') : GlobalVariable.API_VERSION
                    , ('BUSINESS_TAG_ID') : GlobalVariable.BUSINESS_TAG_ID, ('LOGIN_TOKEN') : GlobalVariable.LOGIN_TOKEN
                    , ('CARD_ID') : CARD_IDS[index]]))

        WS.verifyResponseStatusCode(response, 200)

        WS.verifyElementPropertyValue(response, 'error', 'false')

        WS.containsString(response, 'data', false)

        def jsonSlurper = new JsonSlurper()

        def jsonResponse = jsonSlurper.parseText(response.getResponseText())

        def elementsSize = jsonResponse.data.elements.size()

        assert elementsSize > 0

        for (def index1 : (0..elementsSize)) {
            if ((jsonResponse.data.elements[index1]) && (jsonResponse.data.elements[index1].cardtype == 'formcard')) {
                def formContent = jsonSlurper.parseText(jsonResponse.data.elements[index1].content)

                def formData = [:]

                (formData['interactionID']) = 'CommonInteraction_INTERACTION_TYPE_SUBMIT_FORM'

                (formData['categorytype']) = 'FormCard'

                (formData['Flags']) = formContent.Flags

                (formData['MetaData']) = formContent.MetaData

                (formData['FormID']) = formContent.FormID

                (formData['FormSubmissionID']) = formContent.FormSubmissionID

                for (def index2 : (0..formContent.Elements[0].Elements.size())) {
                    if (formContent.Elements[0].Elements[index2]) {
						
						switch (formContent.Elements[0].Elements[index2].ElementType) {
							case 'CHECK_BOX' :
								formData[formContent.Elements[0].Elements[index2].FormMetaID] = 'true'
								break
							case 'EDIT_TEXT' :
								formData[formContent.Elements[0].Elements[index2].FormMetaID] = 'Test Input'
								break
							case 'SPINNER' :
								formData[formContent.Elements[0].Elements[index2].FormMetaID] = formContent.Elements[0].Elements[index2].Options[0]
								break
							default:
								break
						}
                    }
                }
				GlobalVariable.FORM_SUBS.add(JsonOutput.toJson(formData))
            }
        }
    }
}

