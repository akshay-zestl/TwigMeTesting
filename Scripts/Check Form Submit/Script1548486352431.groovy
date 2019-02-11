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

def FORM_SUBS = GlobalVariable.FORM_SUBS

def jsonSlurper = new JsonSlurper()

for (def index : (0..FORM_SUBS.size())) {
    if (FORM_SUBS[index]) {
        def formSub = jsonSlurper.parseText(FORM_SUBS[index])

        response = WS.sendRequest(findTestObject('Form Submit', [('BASE_URL') : GlobalVariable.BASE_URL, ('API_VERSION') : GlobalVariable.API_VERSION
                    , ('BUSINESS_TAG_ID') : GlobalVariable.BUSINESS_TAG_ID, ('LOGIN_TOKEN') : GlobalVariable.LOGIN_TOKEN
                    , ('FORM_ID') : formSub.FormID, ('FORM_SUBMISSION_ID') : formSub.FormSubmissionID, ('FORM_BODY') : FORM_SUBS[
                    index]]))

        WS.verifyResponseStatusCode(response, 200)

        WS.verifyElementPropertyValue(response, 'error', 'false')
    }
}

